use std::{ffi::CString, pin::Pin, str::FromStr, task::Poll};

use crate::{IoCtx, aio::completion::RadosCompletion, librados::rados_aio_getxattr};

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
    pub fn get_xattr<'io, 'data>(
        &'io mut self,
        object: &'data str,
        name: &'data str,
        buf_size: usize,
    ) -> impl Future<Output = Result<Vec<u8>, GetXAttrError>> + Send + 'io
    where
        'data: 'io,
    {
        GetXAttr::new(self, object, name, buf_size)
    }
}

#[derive(Debug)]
struct Data {
    object: CString,
    name: CString,
    buf: Vec<u8>,
}

struct GetXAttr<'io, 'rados, 'data> {
    ctx: &'io IoCtx<'rados>,
    name: &'data str,
    object: &'data str,
    buf_size: usize,
    completion: Option<Option<RadosCompletion<Data>>>,
}

impl<'io, 'rados, 'data> GetXAttr<'io, 'rados, 'data> {
    fn new(io: &'io IoCtx<'rados>, object: &'data str, name: &'data str, buf_size: usize) -> Self {
        Self {
            ctx: io,
            completion: None,
            object,
            name,
            buf_size,
        }
    }
}

impl Future for GetXAttr<'_, '_, '_> {
    type Output = Result<Vec<u8>, GetXAttrError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let buf_size = self.buf_size;
        let name = self.name;
        let object = self.object;
        let ctx = self.ctx.inner;

        let completion = self.completion.get_or_insert_with(|| {
            let data = Data {
                buf: vec![0u8; buf_size],
                name: CString::from_str(name).expect("XAttr name had internal NUL"),
                object: CString::from_str(object).expect("Object name had internal NUL"),
            };

            // SAFETY: the passed-in closure returns `true` if and only
            // if creation of the async operation succeeds.
            unsafe {
                RadosCompletion::new_with(false, data, |completion, data| {
                    let data = &mut *data;

                    // SAFETY: the values passed to this function are
                    // all pointers to pinned values that are available
                    // for the lifetime of `self`, which is also
                    let start = rados_aio_getxattr(
                        ctx,
                        data.object.as_ptr(),
                        completion,
                        data.name.as_ptr(),
                        data.buf.as_mut_ptr() as _,
                        data.buf.len(),
                    );

                    start == 0
                })
            }
        });

        if let Some(completion) = completion {
            completion
                .poll(cx)
                .map_ok(|(len, data)| {
                    let mut vec = data.buf;
                    vec.truncate(len);
                    vec
                })
                .map_err(GetXAttrError::Error)
        } else {
            return Poll::Ready(Err(GetXAttrError::CreateCompletion));
        }
    }
}
