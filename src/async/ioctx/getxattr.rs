use std::{ffi::CString, pin::Pin, task::Poll};

use crate::{IoCtx, r#async::completion::RadosCompletion, librados::rados_aio_getxattr};

#[derive(Debug, Clone)]
pub enum GetXAttrError {
    CreateCompletion,
    Error(i32),
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
    completion: Option<Result<RadosCompletion, ()>>,
    object: CString,
    name: CString,
    output_buf: &'a mut [u8],
    len: Option<usize>,
}

impl<'a> GetXAttr<'a> {
    fn new(io: &'a IoCtx<'a>, object: CString, name: CString, output_buf: &'a mut [u8]) -> Self {
        Self {
            ctx: io,
            completion: None,
            object,
            name,
            output_buf,
            len: None,
        }
    }
}

impl<'a> Future for GetXAttr<'a> {
    type Output = Result<usize, GetXAttrError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if let Some(len) = self.len {
            return Poll::Ready(Ok(len));
        }

        let (output_buf, output_buf_len) = (self.output_buf.as_mut_ptr(), self.output_buf.len());
        let ctx = self.ctx.inner;
        let object = self.object.as_ptr();
        let name = self.name.as_ptr();

        let completion = self.completion.get_or_insert_with(|| unsafe {
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
            .ok_or(())
        });

        let completion = match completion {
            Ok(v) => v,
            Err(_) => return Poll::Ready(Err(GetXAttrError::CreateCompletion)),
        };

        if let Poll::Ready(len) = completion.poll(cx) {
            let len = usize::try_from(len).map_err(|_| GetXAttrError::Error(len))?;
            self.len = Some(len);
            Poll::Ready(Ok(len))
        } else {
            Poll::Pending
        }
    }
}
