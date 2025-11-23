#[allow(warnings)]
mod librados;

use librados::*;

mod aio;
mod bytecount;
mod error;
mod rados;

use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
    mem::MaybeUninit,
};

pub use bytecount::ByteCount;
pub use error::{RadosError, Result};
pub use rados::{ClusterStats, Rados, RadosConfig};

use crate::error::maybe_err;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct PoolStats {
    #[doc = "space used"]
    pub used: ByteCount,
    #[doc = "number of objects in the pool"]
    pub objects: u64,
    #[doc = "number of clones of objects"]
    pub object_clones: u64,
    #[doc = "num_objects * num_replicas"]
    pub object_copies: u64,
    #[doc = "number of objects missing on primary"]
    pub objects_missing_on_primary: u64,
    #[doc = "number of objects found on no OSDs"]
    pub objects_unfound: u64,
    #[doc = "number of objects replicated fewer times than they should be\n (but found on at least one OSD)"]
    pub objects_degraded: u64,
    #[doc = "number of objects read"]
    pub objects_read: u64,
    #[doc = "number of bytes read from objects"]
    pub object_bytes_read: ByteCount,
    #[doc = "number of objects written"]
    pub objects_written: u64,
    #[doc = "amount of bytes written to objects"]
    pub object_bytes_written: ByteCount,
    #[doc = "bytes originally provided by user"]
    pub user_bytes: ByteCount,
    #[doc = "amount of bytes passed to pool"]
    pub compressed_bytes_orig: ByteCount,
    #[doc = "space used by all data after compression"]
    pub compressed_bytes: ByteCount,
    #[doc = "bytes allocated at storage"]
    pub compressed_bytes_alloc: ByteCount,
}

impl From<rados_pool_stat_t> for PoolStats {
    fn from(value: rados_pool_stat_t) -> Self {
        Self {
            used: ByteCount::from_bytes(value.num_bytes),
            objects: value.num_objects,
            object_clones: value.num_object_clones,
            object_copies: value.num_object_copies,
            objects_missing_on_primary: value.num_objects_missing_on_primary,
            objects_unfound: value.num_objects_unfound,
            objects_degraded: value.num_objects_degraded,
            objects_read: value.num_rd,
            object_bytes_read: ByteCount::from_kb(value.num_rd_kb),
            objects_written: value.num_wr,
            object_bytes_written: ByteCount::from_kb(value.num_wr_kb),
            user_bytes: ByteCount::from_bytes(value.num_user_bytes),
            compressed_bytes_orig: ByteCount::from_bytes(value.compressed_bytes_orig),
            compressed_bytes: ByteCount::from_bytes(value.compressed_bytes),
            compressed_bytes_alloc: ByteCount::from_bytes(value.compressed_bytes_alloc),
        }
    }
}

pub struct IoCtx<'rados> {
    inner: rados_ioctx_t,
    _rados: PhantomData<&'rados ()>,
}

unsafe impl Send for IoCtx<'_> {}
unsafe impl Sync for IoCtx<'_> {}

impl<'rados> IoCtx<'rados> {
    pub fn new(rados: &'rados mut Rados, pool: &str) -> Result<Self> {
        let mut inner = std::ptr::null_mut();
        let name = CString::new(pool).unwrap();

        maybe_err(unsafe { rados_ioctx_create(rados.0, name.as_ptr(), &mut inner) })?;

        Ok(Self {
            inner,
            _rados: PhantomData::default(),
        })
    }

    pub fn pool_stats(&mut self) -> Result<PoolStats> {
        let mut stat = MaybeUninit::uninit();
        maybe_err(unsafe { rados_ioctx_pool_stat(self.inner, stat.as_mut_ptr()) })?;
        let stat = unsafe { stat.assume_init() };
        Ok(stat.into())
    }
}

impl Drop for IoCtx<'_> {
    fn drop(&mut self) {
        unsafe { rados_ioctx_destroy(self.inner) };
    }
}

pub struct ExtendedAttributes<'io, 'rados> {
    _io: &'io mut IoCtx<'rados>,
    inner: rados_xattrs_iter_t,
}

unsafe impl<'io, 'rados> Send for ExtendedAttributes<'io, 'rados> where 'rados: 'io {}

impl<'io, 'rados> ExtendedAttributes<'io, 'rados> {
    /// # Safety
    /// `inner` must be a valid, non-null [`rados_xattrs_iter_t`].
    pub(crate) unsafe fn new(io: &'io mut IoCtx<'rados>, inner: rados_xattrs_iter_t) -> Self {
        Self { _io: io, inner }
    }

    pub fn try_next<'a>(&'a mut self) -> Result<Option<(&'a CStr, &'a [u8])>> {
        let mut name = std::ptr::null();
        let mut val = std::ptr::null();
        let mut val_len = 0;

        maybe_err(unsafe { rados_getxattrs_next(self.inner, &mut name, &mut val, &mut val_len) })?;

        if name == std::ptr::null() && val == std::ptr::null() && val_len == 0 {
            Ok(None)
        } else {
            assert!(!name.is_null());
            assert!(!val.is_null());

            let name = unsafe { CStr::from_ptr(name) };
            let val = unsafe { core::slice::from_raw_parts(val as *const u8, val_len) };

            Ok(Some((name, val)))
        }
    }
}

impl Iterator for ExtendedAttributes<'_, '_> {
    type Item = (String, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next()
            .ok()
            .flatten()
            .map(|(k, v)| (k.to_string_lossy().into(), v.to_vec()))
    }
}

impl Drop for ExtendedAttributes<'_, '_> {
    fn drop(&mut self) {
        unsafe { rados_getxattrs_end(self.inner) }
    }
}
