use std::{
    ffi::c_void,
    pin::Pin,
    task::{Context, Poll},
};

use futures::FutureExt;

use crate::{
    Result,
    librados::{
        rados_aio_create_completion, rados_aio_get_return_value, rados_aio_is_complete,
        rados_aio_is_safe, rados_aio_release, rados_completion_t,
    },
};

#[derive(Debug)]
pub struct RadosCompletion<T>
where
    T: 'static,
{
    inner: RadosCompletionInner<T>,
}

unsafe impl<T> Send for RadosCompletion<T> {}

impl<T> RadosCompletion<T>
where
    T: 'static + core::fmt::Debug,
{
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
    /// [`Future`]s should be lazy. That is also the reason that this struct does
    /// not implement [`Future`].
    ///
    /// # Safety
    /// Calling `f` _must only_ return `false` if creation of the underlying completion
    /// operation has failed. If `false` is returned, but the `complete` or `safe`
    /// callback of this [`RadosCompletion`] are called anyways, a double-free will occur.
    ///
    /// Always returning `true` is allowed and does not cause UB. However, it does
    /// cause memory to leak.
    ///
    /// [0]: https://docs.ceph.com/en/latest/rados/api/librados/#c.rados_aio_create_completion
    pub unsafe fn new_with<F>(resolve_on_safe: bool, state: T, f: F) -> Option<Self>
    where
        F: FnOnce(rados_completion_t, Pin<&mut T>) -> bool,
    {
        Some(Self {
            // SAFETY: `RadosCompletion::new_with` has the same safety requirements
            // as `RadosCompletionInner::new_with`.
            inner: unsafe { RadosCompletionInner::new_with(resolve_on_safe, state, f)? },
        })
    }

    pub fn poll(&mut self, cx: &mut Context) -> Poll<Result<(usize, T)>> {
        match self.inner.poll(cx) {
            Poll::Ready((res, state)) => {
                if let Ok(data) = usize::try_from(res) {
                    Poll::Ready(Ok((data, state)))
                } else {
                    Poll::Ready(Err(res.into()))
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[derive(Debug)]
struct RadosCompletionInner<T>
where
    T: 'static,
{
    safe: bool,
    completion: rados_completion_t,
    rx: futures::channel::oneshot::Receiver<T>,
}

impl<T> RadosCompletionInner<T>
where
    T: 'static + core::fmt::Debug,
{
    /// # Safety
    /// See [`RadosCompletion::new_with`].
    unsafe fn new_with<F>(resolve_on_safe: bool, state: T, f: F) -> Option<Self>
    where
        F: FnOnce(rados_completion_t, Pin<&mut T>) -> bool,
    {
        type Tx<T> = futures::channel::oneshot::Sender<T>;
        let (tx, rx): (Tx<T>, _) = futures::channel::oneshot::channel();

        struct State<T> {
            generic: T,
            channel: Tx<T>,
        }

        /// The callback function used to indicate completion.
        unsafe extern "C" fn wake_waker_and_drop_box<T>(_: rados_completion_t, arg: *mut c_void) {
            // SAFETY: `arg` is a type-erased pointer to a `State<Tx>` that
            // is constructed by calling `Box::into_raw`, and `from_raw`
            // is called exactly once for the passed-in value.
            let boxed = unsafe { Box::from_raw(arg as *mut State<T>) };
            let arg = *boxed;
            arg.channel.send(arg.generic).ok();
        }

        let mut completion = std::ptr::null_mut();

        let callback = wake_waker_and_drop_box::<T> as _;
        let (complete, safe) = if resolve_on_safe {
            (None, Some(callback))
        } else {
            (Some(callback), None)
        };

        let state = State {
            generic: state,
            channel: tx,
        };

        let state = Box::leak(Box::new(state));

        // SAFETY: `tx` is valid for the duration of the completion's existence,
        // and the function pointers are valid.
        let completion_created = unsafe {
            rados_aio_create_completion(state as *mut _ as _, complete, safe, &mut completion)
        };

        let generic_state = unsafe { Pin::new_unchecked(&mut state.generic) };

        assert!(
            completion_created == 0,
            "rados_aio_create_completion returned undocumented return code"
        );

        if f(completion, generic_state) {
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
            drop(unsafe { Box::from_raw(state) });
            None
        }
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<(i32, T)> {
        if let Poll::Ready(state) = self.rx.poll_unpin(cx) {
            let Ok(state) = state else { unreachable!() };

            if self.safe && unsafe { rados_aio_is_safe(self.completion) } != 0 {
                let value = unsafe { rados_aio_get_return_value(self.completion) };
                Poll::Ready((value, state))
            } else if unsafe { rados_aio_is_complete(self.completion) } != 0 {
                let value = unsafe { rados_aio_get_return_value(self.completion) };
                Poll::Ready((value, state))
            } else {
                unreachable!()
            }
        } else {
            Poll::Pending
        }
    }
}

impl<T> Drop for RadosCompletionInner<T> {
    fn drop(&mut self) {
        unsafe { rados_aio_release(self.completion) }
    }
}
