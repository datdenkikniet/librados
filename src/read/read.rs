use crate::{IoCtx, Result, error::maybe_err, librados::rados_read_op_read};

use super::read_op::{ReadOp, ReadOpExecutor};

impl IoCtx<'_> {
    pub fn read_blocking(&self, object: &str, bytes: usize, offset: usize) -> Result<Vec<u8>> {
        let executor = ReadOpExecutor::new(self, ReadObject { bytes, offset })?;
        executor.execute(object)
    }

    pub async fn read(&self, object: &str, bytes: usize, offset: usize) -> Result<Vec<u8>> {
        let executor = ReadOpExecutor::new(self, ReadObject { bytes, offset })?;
        executor.execute_async(object).await
    }
}

struct ReadObject {
    bytes: usize,
    offset: usize,
}

impl ReadOp for ReadObject {
    type OperationState = (i32, usize, Vec<u8>);

    type Output = Vec<u8>;

    fn construct_in_place(
        &self,
        read_op: crate::librados::rados_read_op_t,
        mut state: std::pin::Pin<&mut Self::OperationState>,
    ) -> Result<()> {
        state.2 = Vec::with_capacity(self.bytes);

        unsafe {
            rados_read_op_read(
                read_op,
                self.offset as _,
                state.2.capacity(),
                state.2.as_mut_ptr() as _,
                &raw mut state.1,
                &raw mut state.0,
            )
        };

        Ok(())
    }

    fn complete(state: Self::OperationState) -> Result<Self::Output> {
        let (res, len, mut buf) = state;

        maybe_err(res)?;

        assert!(len <= buf.capacity());
        // SAFETY: `len` elements of `buf` are initialized
        // after a succesful operation.
        unsafe { buf.set_len(len) };
        Ok(buf)
    }
}
