use std::{ffi::CString, task::Poll};

use crate::{
    ExtendedAttributes, IoCtx, Result,
    aio::completion::RadosCompletion,
    error::maybe_err,
    librados::{rados_aio_getxattrs, rados_xattrs_iter_t},
};

impl<'rados> IoCtx<'rados> {
    pub async fn get_xattrs<'io, 's>(&'io self, object: &'s str) -> Result<ExtendedAttributes> {
        let mut completion = None;
        let oid = CString::new(object).expect("Object name had interior NUL.");

        let create_completion = || unsafe {
            RadosCompletion::new_with(
                false,
                rados_xattrs_iter_t::default(),
                |completion, mut iter| {
                    maybe_err(rados_aio_getxattrs(
                        self.inner(),
                        oid.as_ptr(),
                        completion,
                        &raw mut *iter,
                    ))
                },
            )
        };

        core::future::poll_fn(|cx| {
            let completion = completion.get_or_insert_with(create_completion);

            match completion {
                Ok(c) => c.poll(cx).map_ok(|(_, iterator)| {
                    assert!(
                        !iterator.is_null(),
                        "Created iterator was null despite future returning Poll::Ready(Ok)"
                    );

                    // SAFETY: `iterator` is not null.
                    unsafe { ExtendedAttributes::new(iterator) }
                }),
                Err(e) => Poll::Ready(Err(e.clone())),
            }
        })
        .await
    }
}
