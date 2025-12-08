use std::{ffi::CStr, pin::Pin};

use crate::{
    IoCtx, Result,
    error::maybe_err,
    librados::{
        rados_omap_get_end, rados_omap_get_next2, rados_omap_iter_t, rados_read_op_omap_get_vals2,
        rados_read_op_t,
    },
};

use super::read_op::{ReadOp, ReadOpExecutor};

#[derive(Debug)]
pub struct OmapKeyValues {
    inner: rados_omap_iter_t,
}

impl OmapKeyValues {
    /// # Safety
    /// `inner` must be a valid and initialized `rados_omap_iter_t`.
    unsafe fn new(inner: rados_omap_iter_t) -> Self {
        Self { inner }
    }

    pub fn try_next(&mut self) -> Result<Option<(&[u8], &[u8])>> {
        let mut key = std::ptr::null_mut();
        let mut val = std::ptr::null_mut();
        let mut key_len = 0;
        let mut val_len = 0;

        maybe_err(unsafe {
            rados_omap_get_next2(self.inner, &mut key, &mut val, &mut key_len, &mut val_len)
        })?;

        if key == std::ptr::null_mut() && val == std::ptr::null_mut() && val_len == 0 {
            Ok(None)
        }
        // Special case: `rados_getxattrs_next` returns `name != NULL && val == NULL && val_len = 0`
        // for xattrs with a value of length 0. However, `core::slice::from_raw_parts` requires a
        // non-null pointer, so we must deal with this case separately.
        else if key != std::ptr::null_mut() && val == std::ptr::null_mut() && val_len == 0 {
            let key = unsafe { core::slice::from_raw_parts(key as _, key_len) };
            let val = unsafe { core::slice::from_raw_parts(std::ptr::dangling(), 0) };

            Ok(Some((key, val)))
        } else {
            assert!(!key.is_null());
            assert!(!val.is_null());

            let key = unsafe { core::slice::from_raw_parts(key as _, key_len) };
            let val = unsafe { core::slice::from_raw_parts(val as *const u8, val_len) };

            Ok(Some((key, val)))
        }
    }
}

impl Iterator for OmapKeyValues {
    type Item = (Vec<u8>, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next()
            .ok()
            .flatten()
            .map(|(k, v)| (k.to_vec(), v.to_vec()))
    }
}

impl Drop for OmapKeyValues {
    fn drop(&mut self) {
        unsafe { rados_omap_get_end(self.inner) };
    }
}

impl<'rados> IoCtx<'rados> {
    pub fn get_omap_vals_blocking(&self, object: &str) -> Result<OmapKeyValues> {
        let operation = OmapGetVals {
            start_after: None,
            filter_prefix: None,
            max_return: None,
        };

        let executor = ReadOpExecutor::new(self, operation)?;

        executor.execute(object)
    }

    pub async fn get_omap_vals(&self, object: &str) -> Result<OmapKeyValues> {
        let operation = OmapGetVals {
            start_after: None,
            filter_prefix: None,
            max_return: None,
        };

        let executor = ReadOpExecutor::new(self, operation)?;

        executor.execute_async(object).await
    }
}

#[derive(Default)]
pub struct State {
    iter: rados_omap_iter_t,
    more: bool,
    status: i32,
}

pub struct OmapGetVals<'a> {
    start_after: Option<&'a CStr>,
    filter_prefix: Option<&'a CStr>,
    max_return: Option<u64>,
}

impl<'a> ReadOp for OmapGetVals<'a> {
    type OperationState = State;

    type Output = OmapKeyValues;

    fn construct_in_place(
        &self,
        read_op: rados_read_op_t,
        mut state: Pin<&mut Self::OperationState>,
    ) -> Result<()> {
        use std::ptr::null;

        unsafe {
            rados_read_op_omap_get_vals2(
                read_op,
                self.start_after.map(|v| v.as_ptr()).unwrap_or(null()),
                self.filter_prefix.map(|v| v.as_ptr()).unwrap_or(null()),
                self.max_return.unwrap_or(u64::MAX),
                &raw mut state.iter,
                &raw mut state.more as _,
                &raw mut state.status as _,
            )
        };

        Ok(())
    }

    fn complete(state: Self::OperationState) -> Result<Self::Output> {
        maybe_err(state.status)?;

        assert!(!state.iter.is_null());

        let res = unsafe { OmapKeyValues::new(state.iter) };
        Ok(res)
    }
}
