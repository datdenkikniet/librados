use std::{
    ffi::CString,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    ExtendedAttributes, IoCtx,
    aio::completion::RadosCompletion,
    librados::{rados_aio_getxattrs, rados_xattrs_iter_t},
};

impl IoCtx<'_> {
    pub fn get_xattrs<'io, 's>(
        &'io mut self,
        object: &'s str,
    ) -> impl Future<Output = Result<ExtendedAttributes<'io>, ()>> + Send {
        let object = CString::new(object).expect("Object name had interior NUL.");

        GetXAttrs::new(self, object)
    }
}

struct GetXAttrs<'a> {
    io: &'a IoCtx<'a>,
    object: CString,
    completion: Option<Option<RadosCompletion>>,
    iterator: rados_xattrs_iter_t,
}

unsafe impl Send for GetXAttrs<'_> {}

impl<'a> GetXAttrs<'a> {
    pub fn new(io: &'a IoCtx<'a>, object: CString) -> Self {
        Self {
            io,
            object,
            completion: None,
            iterator: std::ptr::null_mut(),
        }
    }
}

impl<'a> Future for GetXAttrs<'a> {
    type Output = Result<ExtendedAttributes<'a>, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let io = self.io.inner;
        let oid = self.object.as_ptr();
        let iter = &raw mut self.iterator;

        let completion = self.completion.get_or_insert_with(|| unsafe {
            RadosCompletion::new_with(false, |completion| {
                let create = rados_aio_getxattrs(io, oid, completion, iter);
                create == 0
            })
        });

        if let Some(completion) = completion {
            completion
                .poll(cx)
                .map_ok(|_| {
                    let iterator = std::mem::replace(&mut self.iterator, std::ptr::null_mut());
                    assert!(!iterator.is_null(), "Re-polled completed GetXAttrs future");

                    // SAFETY: `iterator` is not null.
                    unsafe { ExtendedAttributes::new(self.io, iterator) }
                })
                .map_err(|_| ())
        } else {
            Poll::Ready(Err(()))
        }
    }
}
