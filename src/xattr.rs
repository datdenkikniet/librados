use std::ffi::CStr;

use crate::{
    IoCtx, Result,
    error::maybe_err,
    librados::{rados_getxattrs_end, rados_getxattrs_next, rados_xattrs_iter_t},
};

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
