use std::{ffi::CString, pin::Pin, task::Poll};

use crate::{IoCtx, r#async::completion::RadosCompletion, librados::rados_aio_getxattr};

#[derive(Debug, Clone)]
pub enum GetXAttrError {
    CreateCompletion,
    Error(i32),
}

impl From<i32> for GetXAttrError {
    fn from(value: i32) -> Self {
        Self::Error(value)
    }
}

impl IoCtx<'_> {
    pub fn get_xattr<'a>(
        &'a mut self,
        object: &str,
        name: &str,
        buffer: &'a mut [u8],
    ) -> impl Future<Output = Result<usize, GetXAttrError>> + 'a {
        let object = CString::new(object).expect("Object name had interior NUL");
        let name = CString::new(name).expect("XAttr name had internal NUL");

        GetXAttr::new(self, object, name, buffer)
    }
}

struct GetXAttr<'a> {
    ctx: &'a IoCtx<'a>,
    object: CString,
    name: CString,
    output_buf: &'a mut [u8],
    completion: Option<Option<RadosCompletion>>,
}

impl<'a> GetXAttr<'a> {
    fn new(io: &'a IoCtx<'a>, object: CString, name: CString, output_buf: &'a mut [u8]) -> Self {
        Self {
            ctx: io,
            completion: None,
            object,
            name,
            output_buf,
        }
    }
}

impl<'a> Future for GetXAttr<'a> {
    type Output = Result<usize, GetXAttrError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let (output_buf, output_buf_len) = (self.output_buf.as_mut_ptr(), self.output_buf.len());
        let ctx = self.ctx.inner;
        let object = self.object.as_ptr();
        let name = self.name.as_ptr();

        let completion = self.completion.get_or_insert_with(|| {
            // SAFETY: the passed-in closure returns `true` if and only
            // if creation of the async operation succeeds.
            unsafe {
                RadosCompletion::new_with(false, |completion| {
                    // SAFETY: the values passed to this function are
                    // all pointers to pinned values that are available
                    // for the lifetime of `self`, which is also
                    let start = rados_aio_getxattr(
                        ctx,
                        object,
                        completion,
                        name,
                        output_buf as _,
                        output_buf_len,
                    );

                    start == 0
                })
            }
        });

        if let Some(completion) = completion {
            completion.poll(cx)
        } else {
            return Poll::Ready(Err(GetXAttrError::CreateCompletion));
        }
    }
}
