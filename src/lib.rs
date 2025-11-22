#[allow(warnings)]
mod librados;

use librados::*;

mod aio;
mod rados;

use std::{ffi::CString, marker::PhantomData};

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
