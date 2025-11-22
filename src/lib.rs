#[allow(warnings)]
mod librados;

use librados::*;

mod aio;
mod rados;

use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
};

pub use rados::{Rados, RadosConfig};

pub struct IoCtx<'rados> {
    inner: rados_ioctx_t,
    _rados: PhantomData<&'rados Rados>,
}

impl<'rados> IoCtx<'rados> {
    pub fn new(rados: &'rados Rados, pool: &str) -> Option<Self> {
        let mut inner = std::ptr::null_mut();
        let name = CString::new(pool).unwrap();

        if unsafe { rados_ioctx_create(rados.0, name.as_ptr(), &mut inner) } == 0 {
            Some(Self {
                inner,
                _rados: PhantomData::default(),
            })
        } else {
            None
        }
    }
}

impl Drop for IoCtx<'_> {
    fn drop(&mut self) {
        unsafe { rados_ioctx_destroy(self.inner) };
    }
}

pub struct XattrIterator<'a> {
    _io: &'a IoCtx<'a>,
    inner: rados_xattrs_iter_t,
}

impl<'a> XattrIterator<'a> {
    /// # Safety
    /// `inner` must be a valid, non-null [`rados_xattrs_iter_t`].
    pub(crate) unsafe fn new(io: &'a IoCtx<'a>, inner: rados_xattrs_iter_t) -> Self {
        Self { _io: io, inner }
    }

    pub fn try_next(&mut self) -> Result<Option<(String, Vec<u8>)>, i32> {
        let mut name = std::ptr::null();
        let mut val = std::ptr::null();
        let mut val_len = 0;

        let get = unsafe { rados_getxattrs_next(self.inner, &mut name, &mut val, &mut val_len) };

        if get != 0 {
            Err(get)
        } else if name == std::ptr::null() && val == std::ptr::null() && val_len == 0 {
            Ok(None)
        } else {
            let name = unsafe { CStr::from_ptr(name) }.to_string_lossy().into();
            let val = unsafe { core::slice::from_raw_parts(val as *const u8, val_len) }.to_vec();

            Ok(Some((name, val)))
        }
    }
}

impl Iterator for XattrIterator<'_> {
    type Item = (String, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        self.try_next().ok().flatten()
    }
}

impl Drop for XattrIterator<'_> {
    fn drop(&mut self) {
        unsafe { rados_getxattrs_end(self.inner) }
    }
}
