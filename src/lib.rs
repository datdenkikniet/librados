#[allow(warnings)]
mod librados;

pub use librados::*;

use std::{
    ffi::{CStr, CString, c_void},
    marker::PhantomData,
    pin::Pin,
    task::Poll,
};

use futures::FutureExt;

pub struct IoCtx<'rados> {
    inner: rados_ioctx_t,
    rados: PhantomData<&'rados ()>,
}

impl<'rados> IoCtx<'rados> {
    pub fn new(cluster: &'rados librados::rados_t, name: &str) -> Option<Self> {
        let mut inner = std::ptr::null_mut();
        let name = CString::new(name).unwrap();

        if unsafe { rados_ioctx_create(*cluster, name.as_ptr(), &mut inner) } == 0 {
            Some(Self {
                inner,
                rados: Default::default(),
            })
        } else {
            None
        }
    }

    pub fn get_xattr<'a>(
        &'a mut self,
        object: &'a CStr,
        name: &'a CStr,
    ) -> impl Future<Output = Result<Vec<u8>, ()>> + 'a {
        GetXAttr::new(self, object, name)
    }
}

impl Drop for IoCtx<'_> {
    fn drop(&mut self) {
        unsafe { rados_ioctx_destroy(self.inner) };
    }
}

struct GetXAttr<'a> {
    ctx: &'a IoCtx<'a>,
    completion: Option<Result<RadosCompletion, ()>>,
    object: &'a CStr,
    name: &'a CStr,
    output_buf: Vec<u8>,
    completed: bool,
}

impl<'a> GetXAttr<'a> {
    fn new(io: &'a IoCtx<'a>, object: &'a CStr, name: &'a CStr) -> Self {
        let output_buf = vec![0u8; 128];

        Self {
            ctx: io,
            completion: None,
            object,
            name,
            output_buf,
            completed: false,
        }
    }
}

impl Future for GetXAttr<'_> {
    type Output = Result<Vec<u8>, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if self.completed {
            return Poll::Ready(Ok(Vec::new()));
        }

        let (output_buf, output_buf_len) = (self.output_buf.as_mut_ptr(), self.output_buf.len());
        let ctx = self.ctx.inner;
        let object = self.object.as_ptr();
        let name = self.name.as_ptr();

        let completion = self.completion.get_or_insert_with(|| {
            let completion = RadosCompletion::new();

            let len = unsafe {
                rados_aio_getxattr(
                    ctx,
                    object,
                    completion.completion,
                    name,
                    output_buf as _,
                    output_buf_len,
                )
            };

            (len == 0).then_some(completion).ok_or(())
        });

        let completion = match completion {
            Ok(v) => v,
            Err(_) => return Poll::Ready(Err(())),
        };

        if let Poll::Ready(len) = completion.poll(cx) {
            self.completed = true;
            let len = usize::try_from(len).map_err(|_| ())?;
            let mut data = core::mem::take(&mut self.output_buf);
            data.truncate(len);
            Poll::Ready(Ok(data))
        } else {
            Poll::Pending
        }
    }
}

struct RadosCompletion {
    safe: bool,
    completion: rados_completion_t,
    rx: futures::channel::oneshot::Receiver<()>,
}

impl RadosCompletion {
    pub fn new() -> Self {
        let safe = false;

        let (tx, rx) = futures::channel::oneshot::channel();

        let tx = Box::leak(Box::new(tx));

        unsafe extern "C" fn wake_waker_and_drop_box(_: rados_completion_t, arg: *mut c_void) {
            let arg = unsafe { &mut *(arg as *mut futures::channel::oneshot::Sender<()>) };
            let arg = *unsafe { Box::from_raw(arg) };
            arg.send(()).ok();
        }

        let mut completion = std::ptr::null_mut();

        let (complete, safe) = if safe {
            (None, Some(wake_waker_and_drop_box as _))
        } else {
            (Some(wake_waker_and_drop_box as _), None)
        };

        let completion_created = unsafe {
            rados_aio_create_completion(tx as *mut _ as _, complete, safe, &mut completion)
        };

        assert!(
            completion_created == 0,
            "rados_aio_create_completion returned undocumented return code"
        );

        Self {
            safe: false,
            completion,
            rx,
        }
    }

    fn poll(&mut self, cx: &mut std::task::Context<'_>) -> core::task::Poll<i32> {
        if self.rx.poll_unpin(cx).is_ready() {
            if self.safe && unsafe { rados_aio_is_safe(self.completion) } != 0 {
                let value = unsafe { rados_aio_get_return_value(self.completion) };
                Poll::Ready(value)
            } else if unsafe { rados_aio_is_complete(self.completion) } != 0 {
                let value = unsafe { rados_aio_get_return_value(self.completion) };
                Poll::Ready(value)
            } else {
                Poll::Pending
            }
        } else {
            Poll::Pending
        }
    }
}

impl Drop for RadosCompletion {
    fn drop(&mut self) {
        unsafe { rados_aio_release(self.completion) }
    }
}
