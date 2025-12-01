use std::ffi::CString;

use crate::{
    ExtendedAttributes, IoCtx, RadosCompletion, Result,
    error::maybe_err,
    librados::{rados_aio_getxattrs, rados_xattrs_iter_t},
};

impl<'rados> IoCtx<'rados> {
    pub async fn get_xattrs<'io, 's>(&'io self, object: &'s str) -> Result<ExtendedAttributes> {
        let oid = CString::new(object).expect("Object name had interior NUL.");

        let completion = unsafe {
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
            )?
        };

        let (_, iterator) = completion.wait_for().await?;

        assert!(
            !iterator.is_null(),
            "Created iterator was null despite future returning Poll::Ready(Ok)"
        );

        // SAFETY: `iterator` is not null.
        Ok(unsafe { ExtendedAttributes::new(iterator) })
    }
}
