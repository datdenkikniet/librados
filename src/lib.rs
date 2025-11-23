#[allow(warnings)]
mod librados;

use librados::*;

mod aio;
mod rados;

use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
};

pub use rados::{Rados, RadosConfig};

pub struct IoCtx<'rados> {
    inner: rados_ioctx_t,
    _rados: PhantomData<&'rados ()>,
}

unsafe impl Send for IoCtx<'_> {}
unsafe impl Sync for IoCtx<'_> {}

impl<'rados> IoCtx<'rados> {
    pub fn new(rados: &'rados mut Rados, pool: &str) -> Option<Self> {
        let mut inner = std::ptr::null_mut();
        let name = CString::new(pool).unwrap();

        if unsafe { rados_ioctx_create(rados.0, name.as_ptr(), &mut inner) } == 0 {
            Some(Self {
                inner,
                _rados: PhantomData::default(),
            })
        } else {
            None
        }
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

    pub fn try_next<'a>(&'a mut self) -> Result<Option<(&'a CStr, &'a [u8])>, i32> {
        let mut name = std::ptr::null();
        let mut val = std::ptr::null();
        let mut val_len = 0;

        let get = unsafe { rados_getxattrs_next(self.inner, &mut name, &mut val, &mut val_len) };

        if get != 0 {
            Err(get)
        } else if name == std::ptr::null() && val == std::ptr::null() && val_len == 0 {
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
