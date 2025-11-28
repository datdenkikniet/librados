use std::{ffi::CString, task::Poll};

use crate::{
    IoCtx, Result, aio::completion::RadosCompletion, error::maybe_err, librados::rados_aio_getxattr,
};

impl IoCtx<'_> {
    pub async fn get_xattr(&self, object: &str, name: &str, buf_size: usize) -> Result<Vec<u8>> {
        let mut completion = None;
        let object = CString::new(object).expect("Object ID contained internal NUL");
        let name = CString::new(name).expect("Name contained internal NUL");

        let create_completion = || {
            let data_buf = vec![0u8; buf_size];
            // SAFETY: the passed-in closure returns `true` if and only
            // if creation of the async operation succeeds.
            unsafe {
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
                })
            }
        };

        core::future::poll_fn(|cx| {
            let completion = completion.get_or_insert_with(create_completion);

            match completion {
                Ok(c) => c.poll(cx).map_ok(|(len, mut buf)| {
                    buf.truncate(len);
                    buf
                }),
                Err(e) => Poll::Ready(Err(e.clone())),
            }
        })
        .await
    }
}
