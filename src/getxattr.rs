use std::ffi::CString;

use crate::{
    IoCtx, RadosCompletion, Result,
    error::{maybe_err, maybe_err_or_val},
    librados::{rados_aio_getxattr, rados_getxattr},
};

impl IoCtx<'_> {
    /// Blockingly get the value of extended attribute
    /// `name` for object `object`, using a buffer of `buf_size`
    /// bytes to store the result.
    ///
    /// # Panics
    /// This function panics of `object` or `name` have internal `0` bytes.
    ///
    // TODO: what does this return if `buf_size` is too small?
    pub fn get_xattr_blocking(&self, object: &str, name: &str, buf_size: usize) -> Result<Vec<u8>> {
        let object = CString::new(object).expect("Object ID contained internal NUL");
        let name = CString::new(name).expect("Name contained internal NUL");

        let mut data_buf = vec![0u8; buf_size];

        let len = maybe_err_or_val(unsafe {
            rados_getxattr(
                self.inner(),
                object.as_ptr(),
                name.as_ptr(),
                data_buf.as_mut_ptr() as _,
                data_buf.len(),
            )
        })?;

        data_buf.truncate(len as _);

        Ok(data_buf)
    }

    /// Get the value of extended attribute `name`
    /// for object `object`, using a buffer of `buf_size`
    /// bytes to store the result.
    ///
    /// # Panics
    /// This function panics of `object` or `name` have internal `0` bytes.
    ///
    // TODO: what does this return if `buf_size` is too small?
    pub async fn get_xattr(&self, object: &str, name: &str, buf_size: usize) -> Result<Vec<u8>> {
        let object = CString::new(object).expect("Object ID contained internal NUL");
        let name = CString::new(name).expect("Name contained internal NUL");

        let data_buf = vec![0u8; buf_size];

        // SAFETY: the passed-in closure returns `true` if and only
        // if creation of the async operation succeeds.
        let completion = unsafe {
            RadosCompletion::new_with(false, data_buf, |completion, mut data_buf| {
                // SAFETY: the values passed to this function are
                // all pointers to pinned values that are available
                // for the lifetime of `self`, which is also
                maybe_err(rados_aio_getxattr(
                    self.inner(),
                    object.as_ptr(),
                    completion,
                    name.as_ptr(),
                    data_buf.as_mut_ptr() as _,
                    data_buf.len(),
                ))
            })?
        };

        let (len, mut buf) = completion.wait_for().await?;

        buf.truncate(len);

        Ok(buf)
    }
}
