//! Wrapped access to [`rados_ioctx_t`].
//!
//! Access to data owned by an [`rados_ioctx_t`] through `librados` functions
//! is thread-safe, so handing immutable references, and implementing both
//! `Send` and `Sync` for this type are OK, provided that the pointer is bound
//! to the lifetime of the underpinning [`rados_t`].
//!
//! The only exception is managing configuration of the [`rados_ioctx_t`] itself ([ref][0]).
//! This is solved by requiring exclusive access through an `&mut self` receiver.
//!
//! `librados` was made almost fully thread-safe back in [`v0.53`][1].
//!
//! [0]: https://docs.ceph.com/en/reef/rados/api/librados/#c.rados_ioctx_t
//! [1]: https://tracker.ceph.com/issues/2525

use std::{ffi::CString, marker::PhantomData, mem::MaybeUninit};

use crate::{
    ByteCount, Rados, Result,
    error::{maybe_err, maybe_err_or_val},
    librados::{
        LIBRADOS_ALL_NSPACES, rados_ioctx_create, rados_ioctx_destroy, rados_ioctx_get_namespace,
        rados_ioctx_pool_stat, rados_ioctx_set_namespace, rados_ioctx_t, rados_pool_stat_t,
        rados_t,
    },
};

pub enum Namespace {
    Named(String),
    All,
}

#[derive(Debug)]
pub struct IoCtx<'rados> {
    inner: rados_ioctx_t,
    _rados: PhantomData<&'rados mut rados_t>,
}

unsafe impl Send for IoCtx<'_> {}
unsafe impl Sync for IoCtx<'_> {}

impl<'rados> IoCtx<'rados> {
    pub fn new(rados: &'rados mut Rados, pool: &str) -> Result<Self> {
        let mut inner = std::ptr::null_mut();
        let name = CString::new(pool).unwrap();

        maybe_err(unsafe { rados_ioctx_create(rados.inner(), name.as_ptr(), &mut inner) })?;

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

    pub fn set_namespace(&mut self, ns: &Namespace) {
        let ns = match ns {
            Namespace::Named(n) => {
                CString::new(n.as_str()).expect("Namespace contained interior NUL")
            }
            Namespace::All => CString::from_vec_with_nul(LIBRADOS_ALL_NSPACES.to_vec()).unwrap(),
        };

        unsafe { rados_ioctx_set_namespace(self.inner, ns.as_ptr()) };
    }

    pub fn get_namespace(&self, max_len: usize) -> Result<Namespace> {
        let mut str = Vec::with_capacity(max_len);

        let len = maybe_err_or_val(unsafe {
            rados_ioctx_get_namespace(self.inner, str.as_mut_ptr() as _, str.capacity() as _)
        })?;

        unsafe { str.set_len(len as _) };

        if str == LIBRADOS_ALL_NSPACES {
            Ok(Namespace::All)
        } else {
            let ns = String::from_utf8(str).expect("Namespace is not valid UTF8");
            Ok(Namespace::Named(ns))
        }
    }

    pub(crate) fn inner(&self) -> rados_ioctx_t {
        self.inner
    }
}

impl Drop for IoCtx<'_> {
    fn drop(&mut self) {
        unsafe { rados_ioctx_destroy(self.inner) };
    }
}

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
