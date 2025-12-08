use crate::{
    ByteCount, IoCtx, Result,
    error::maybe_err,
    librados::{rados_read_op_stat2, timespec},
};

use super::read_op::{ReadOp, ReadOpExecutor};

#[derive(Debug, Clone, Copy)]
pub struct Stat {
    pub size: ByteCount,
    pub mtime: timespec,
}

impl IoCtx<'_> {
    pub fn stat_blocking(&self, obj: &str) -> Result<Stat> {
        let executor = ReadOpExecutor::new(self, StatOp)?;
        executor.execute(obj)
    }

    pub async fn stat(&self, obj: &str) -> Result<Stat> {
        let executor = ReadOpExecutor::new(self, StatOp)?;
        executor.execute_async(obj).await
    }
}

struct StatOp;

struct State {
    result: i32,
    size: u64,
    mtime: timespec,
}

impl Default for State {
    fn default() -> Self {
        Self {
            size: Default::default(),
            mtime: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            result: Default::default(),
        }
    }
}

impl ReadOp for StatOp {
    type OperationState = State;

    type Output = Stat;

    fn construct_in_place(
        &self,
        read_op: crate::librados::rados_read_op_t,
        mut state: std::pin::Pin<&mut Self::OperationState>,
    ) -> crate::Result<()> {
        unsafe {
            rados_read_op_stat2(
                read_op,
                &raw mut state.size,
                &raw mut state.mtime,
                &raw mut state.result,
            )
        };

        Ok(())
    }

    fn complete(state: Self::OperationState) -> crate::Result<Self::Output> {
        maybe_err(state.result)?;

        Ok(Stat {
            size: ByteCount::from_bytes(state.size),
            mtime: state.mtime,
        })
    }
}
