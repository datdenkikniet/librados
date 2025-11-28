use std::{ffi::CString, pin::Pin, task::Poll};

use crate::{
    IoCtx, RadosError, Result,
    aio::completion::RadosCompletion,
    error::maybe_err,
    librados::{
        rados_aio_read_op_operate, rados_create_read_op, rados_read_op_operate, rados_read_op_t,
        rados_release_read_op,
    },
};

struct ReadOpPtr(rados_read_op_t);

impl ReadOpPtr {
    fn new() -> Result<Self> {
        let read_op = unsafe { rados_create_read_op() };

        (!read_op.is_null())
            .then_some(Self(read_op))
            .ok_or(RadosError::Unknown(-1))
    }

    fn get(&self) -> rados_read_op_t {
        self.0
    }
}

impl Drop for ReadOpPtr {
    fn drop(&mut self) {
        unsafe { rados_release_read_op(self.0) };
    }
}

pub(crate) struct ReadOpExecutor<'rados, 'ioctx, T>
where
    T: ReadOp,
{
    operation: T,
    inner: ReadOpPtr,
    ioctx: &'ioctx IoCtx<'rados>,
}

impl<'rados, 'ioctx, T> ReadOpExecutor<'rados, 'ioctx, T>
where
    T: ReadOp,
{
    pub fn new(ioctx: &'ioctx IoCtx<'rados>, operation: T) -> Result<Self> {
        Ok(Self {
            inner: ReadOpPtr::new()?,
            operation,
            ioctx,
        })
    }

    pub fn execute(self, object: &str) -> Result<T::Output> {
        let object = CString::new(object).expect("Object ID had interior NUL.");
        let mut output = T::OperationState::default();

        let pinned = unsafe { Pin::new_unchecked(&mut output) };

        self.operation.construct(self.inner.get(), pinned)?;

        let result = unsafe {
            rados_read_op_operate(self.inner.get(), self.ioctx.inner(), object.as_ptr(), 0)
        };

        maybe_err(result)?;

        T::complete(output)
    }

    pub async fn execute_async(self, object: &str) -> Result<T::Output>
    where
        T::OperationState: 'static + Unpin,
    {
        let mut completion = None;

        let (_, (_, output)) = core::future::poll_fn(|cx| {
            let completion = completion.get_or_insert_with(|| unsafe {
                let object = CString::new(object).expect("Object ID had interior NUL");
                let state = T::OperationState::default();

                RadosCompletion::new_with(false, (object, state), |completion, mut full_state| {
                    let pinned = &mut full_state.1;
                    let op_state = core::pin::Pin::new_unchecked(pinned);
                    self.operation.construct(self.inner.get(), op_state)?;

                    maybe_err(rados_aio_read_op_operate(
                        self.inner.get(),
                        self.ioctx.inner(),
                        completion,
                        full_state.0.as_ptr(),
                        0,
                    ))
                })
            });

            let completion = match completion {
                Ok(c) => c,
                Err(e) => return Poll::Ready(Err(e.clone())),
            };

            completion.poll(cx)
        })
        .await?;

        T::complete(output)
    }
}

pub trait ReadOp
where
    Self: Sized,
{
    type OperationState: Default;
    type Output;

    fn construct(
        &self,
        read_op: rados_read_op_t,
        state: Pin<&mut Self::OperationState>,
    ) -> Result<()>;

    fn complete(state: Self::OperationState) -> Result<Self::Output>;
}
