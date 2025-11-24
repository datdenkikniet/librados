use std::{
    ffi::CString,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    ExtendedAttributes, IoCtx, Result,
    aio::completion::RadosCompletion,
    librados::{rados_aio_getxattrs, rados_xattrs_iter_t},
};

impl<'rados> IoCtx<'rados> {
    pub fn get_xattrs<'io, 's>(
        &'io mut self,
        object: &'s str,
    ) -> impl Future<Output = Result<ExtendedAttributes<'io, 'rados>>> + Send {
        let object = CString::new(object).expect("Object name had interior NUL.");
        GetXAttrs::new(self, object)
    }
}

struct GetXAttrs<'io, 'rados> {
    io: Option<&'io mut IoCtx<'rados>>,
    object: CString,
    completion: Option<Option<RadosCompletion<rados_xattrs_iter_t>>>,
}

unsafe impl<'io, 'rados> Send for GetXAttrs<'io, 'rados> {}

impl<'io, 'rados> GetXAttrs<'io, 'rados> {
    pub fn new(io: &'io mut IoCtx<'rados>, object: CString) -> Self {
        Self {
            io: Some(io),
            object,
            completion: None,
        }
    }
}

impl<'io, 'rados> Future for GetXAttrs<'io, 'rados> {
    type Output = Result<ExtendedAttributes<'io, 'rados>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        const MSG: &'static str = "Re-polled completed GetXAttrs future";

        let io = self.io.as_mut().expect(MSG).inner();

        let oid = self.object.as_ptr();

        let completion = self.completion.get_or_insert_with(|| unsafe {
            RadosCompletion::new_with(
                false,
                rados_xattrs_iter_t::default(),
                |completion, mut iter| {
                    let create = rados_aio_getxattrs(io, oid, completion, &raw mut *iter);
                    create == 0
                },
            )
        });

        if let Some(completion) = completion {
            completion.poll(cx).map_ok(move |(_, iterator)| {
                assert!(
                    !iterator.is_null(),
                    "Created iterator was null despite future returning Poll::Ready(Ok)"
                );

                // SAFETY: `iterator` is not null.
                unsafe { ExtendedAttributes::new(self.io.take().expect(MSG), iterator) }
            })
        } else {
            Poll::Ready(Err(i32::MIN.into()))
        }
    }
}
