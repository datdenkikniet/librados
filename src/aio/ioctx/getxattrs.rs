use std::{
    ffi::CString,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    ExtendedAttributes, IoCtx, Result,
    aio::completion::RadosCompletion,
    error::maybe_err,
    librados::{rados_aio_getxattrs, rados_xattrs_iter_t},
};

impl<'rados> IoCtx<'rados> {
    pub fn get_xattrs<'io, 's>(
        &'io self,
        object: &'s str,
    ) -> impl Future<Output = Result<ExtendedAttributes>> + Send {
        let object = CString::new(object).expect("Object name had interior NUL.");
        GetXAttrs::new(self, object)
    }
}

struct GetXAttrs<'io, 'rados> {
    io: &'io IoCtx<'rados>,
    object: CString,
    completion: Option<crate::Result<RadosCompletion<rados_xattrs_iter_t>>>,
}

unsafe impl<'io, 'rados> Send for GetXAttrs<'io, 'rados> {}

impl<'io, 'rados> GetXAttrs<'io, 'rados> {
    pub fn new(io: &'io IoCtx<'rados>, object: CString) -> Self {
        Self {
            io,
            object,
            completion: None,
        }
    }
}

impl<'io, 'rados> Future for GetXAttrs<'io, 'rados> {
    type Output = Result<ExtendedAttributes>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let io = self.io.inner();

        let oid = self.object.as_ptr();

        let completion = self.completion.get_or_insert_with(|| unsafe {
            RadosCompletion::new_with(
                false,
                rados_xattrs_iter_t::default(),
                |completion, mut iter| {
                    maybe_err(rados_aio_getxattrs(io, oid, completion, &raw mut *iter))
                },
            )
        });

        if let Ok(completion) = completion {
            completion.poll(cx).map_ok(move |(_, iterator)| {
                assert!(
                    !iterator.is_null(),
                    "Created iterator was null despite future returning Poll::Ready(Ok)"
                );

                // SAFETY: `iterator` is not null.
                unsafe { ExtendedAttributes::new(iterator) }
            })
        } else {
            Poll::Ready(Err(i32::MIN.into()))
        }
    }
}
