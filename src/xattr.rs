use std::ffi::CStr;

use crate::{
    Result,
    error::maybe_err,
    librados::{rados_getxattrs_end, rados_getxattrs_next, rados_xattrs_iter_t},
};

/// An iterator over the extended attributes of a file.
pub struct ExtendedAttributes {
    /// The underlying `rados_xattrs_iter_t`.
    ///
    /// This pointer is the effective equivalent of an
    /// `std::vec::IntoIter<(&CStr, &[u8])>`, with the
    /// important caveat that the returned references
    /// are only valid until the next iteration.
    inner: rados_xattrs_iter_t,
}

unsafe impl Send for ExtendedAttributes {}

impl ExtendedAttributes {
    /// # Safety
    /// `inner` must be a valid, non-null [`rados_xattrs_iter_t`].
    pub(crate) unsafe fn new(inner: rados_xattrs_iter_t) -> Self {
        Self { inner }
    }

    /// Try to read the next item in the list.
    ///
    /// This function will return `Err` if the underlying operation
    /// fails. Currently, the only way for this to happen is for `ENOMEM`
    /// to be returned.
    pub fn try_next<'a>(&'a mut self) -> Result<Option<(&'a CStr, &'a [u8])>> {
        let mut name = std::ptr::null();
        let mut val = std::ptr::null();
        let mut val_len = 0;

        maybe_err(unsafe { rados_getxattrs_next(self.inner, &mut name, &mut val, &mut val_len) })?;

        if name == std::ptr::null() && val == std::ptr::null() && val_len == 0 {
            Ok(None)
        }
        // Special case: `rados_getxattrs_next` returns `name != NULL && val == NULL && val_len = 0`
        // for xattrs with a value of length 0. However, `core::slice::from_raw_parts` requires a
        // non-null pointer, so we must deal with this case separately.
        else if name != std::ptr::null() && val == std::ptr::null() && val_len == 0 {
            let name = unsafe { CStr::from_ptr(name) };
            let val = unsafe { core::slice::from_raw_parts(std::ptr::dangling(), 0) };

            Ok(Some((name, val)))
        } else {
            assert!(!name.is_null());
            assert!(!val.is_null());

            let name = unsafe { CStr::from_ptr(name) };
            let val = unsafe { core::slice::from_raw_parts(val as *const u8, val_len) };

            Ok(Some((name, val)))
        }
    }
}

impl Iterator for ExtendedAttributes {
    type Item = (String, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next()
            .ok()
            .flatten()
            .map(|(k, v)| (k.to_string_lossy().into(), v.to_vec()))
    }
}

impl Drop for ExtendedAttributes {
    fn drop(&mut self) {
        unsafe { rados_getxattrs_end(self.inner) }
    }
}
