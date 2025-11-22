use std::{ffi::c_void, task::Poll};

use futures::FutureExt;

use crate::librados::{
    rados_aio_create_completion, rados_aio_get_return_value, rados_aio_is_complete,
    rados_aio_is_safe, rados_aio_release, rados_completion_t,
};

pub struct RadosCompletion {
    safe: bool,
    completion: rados_completion_t,
    rx: futures::channel::oneshot::Receiver<()>,
}

impl RadosCompletion {
    /// Create a new [`RadosCompletion`].
    ///
    /// The `resolve_on_safe` argument controls the stage at which the
    /// the created [`RadosCompletion`] is considered to be
    /// "done": on the `complete` callback if `!resolve_on_safe`, and
    /// on the `safe` callback if `resolve_on_safe`.
    ///
    /// It is important to note that some librados async operations
    /// _never_ call the `safe` callback, so restricting the inputs
    /// to this value appropriatly is important to avoid memory leaks.
    ///
    /// The `f` function is responsible for creating the actual operation
    /// that this completion will track: it must return a boolean indicating
    /// success.
    ///
    /// The lifetime of the [`rados_completion_t`] passed to `F` is exactly equal
    /// to the lifetime of the returned [`RadosCompletion`], but you _must not_
    /// use it outside of the closure.
    ///
    /// For more information about the respective callbacks, see: [`rados_aio_create_completion`][0]
    ///
    /// > **Note**: this function should only be called _during_ a `poll` operation. Since it creates and kicks off
    /// a completion immediately, setting it up during the creation of a [`Future`] is semantically incorrect, as
    /// [`Future`]s should be inactive until they are polled. That is also the reason that this struct does
    /// not implement [`Future`].
    ///
    /// # Safety
    /// If `F` returns `false`, creation of the underlying completion
    /// operation _must_ have failed. If it has succeeded, and the `complete`
    /// or `safe` callback of this [`RadosCompletion`] are called, a double-free
    /// will occur.
    ///
    /// [0]: https://docs.ceph.com/en/latest/rados/api/librados/#c.rados_aio_create_completion
    pub unsafe fn new_with<F>(resolve_on_safe: bool, f: F) -> Option<Self>
    where
        F: FnOnce(rados_completion_t) -> bool,
    {
        type Tx = futures::channel::oneshot::Sender<()>;
        let (tx, rx): (Tx, _) = futures::channel::oneshot::channel();

        /// The callback function used to indicate completion.
        unsafe extern "C" fn wake_waker_and_drop_box(_: rados_completion_t, arg: *mut c_void) {
            // SAFETY: `arg` is a type-erased pointer to a `Tx` that
            // is constructed by calling `Box::into_raw`, and `from_raw`
            // is called exactly once for the passed-in value.
            let boxed = unsafe { Box::from_raw(arg as *mut Tx) };
            let arg = *boxed;
            arg.send(()).ok();
        }

        let mut completion = std::ptr::null_mut();

        let (complete, safe) = if resolve_on_safe {
            (None, Some(wake_waker_and_drop_box as _))
        } else {
            (Some(wake_waker_and_drop_box as _), None)
        };

        let tx = Box::into_raw(Box::new(tx));

        // SAFETY: `tx` is valid for the duration of the completion's existence,
        // and the function pointers are valid.
        let completion_created =
            unsafe { rados_aio_create_completion(tx as _, complete, safe, &mut completion) };

        assert!(
            completion_created == 0,
            "rados_aio_create_completion returned undocumented return code"
        );

        if f(completion) {
            Some(Self {
                safe: false,
                completion,
                rx,
            })
        } else {
            // Creation of completion operation failed, so the wake-and-drop
            // callback will never be called.
            //
            // To prevent memory leaks, recreate the box containing the
            // sender and drop it.
            // SAFETY: `arg` is a pointer that is constructed by calling
            // `Box::into_raw`, and `from_raw` is called exactly once
            // for the pointer.
            drop(unsafe { Box::from_raw(tx) });
            None
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
