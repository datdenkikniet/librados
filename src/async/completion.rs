use std::{ffi::c_void, task::Poll};

use futures::FutureExt;

use crate::librados::{
    rados_aio_create_completion, rados_aio_get_return_value, rados_aio_is_complete,
    rados_aio_is_safe, rados_aio_release, rados_completion_t,
};

pub struct RadosCompletion {
    safe: bool,
    pub completion: rados_completion_t,
    rx: futures::channel::oneshot::Receiver<()>,
}

impl RadosCompletion {
    pub fn new(safe: bool) -> Self {
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

    pub fn poll(&mut self, cx: &mut std::task::Context<'_>) -> core::task::Poll<i32> {
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
