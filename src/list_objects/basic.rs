use std::marker::PhantomData;

use crate::{
    IoCtx, Object, RadosError, Result,
    error::maybe_err,
    librados::{
        rados_list_ctx_t, rados_nobjects_list_close, rados_nobjects_list_get_pg_hash_position,
        rados_nobjects_list_next2, rados_nobjects_list_open, rados_nobjects_list_seek,
        rados_object_list_item,
    },
};

pub struct OwnedObject {
    oid: Vec<u8>,
    locator: Vec<u8>,
    nspace: Vec<u8>,
}

impl OwnedObject {
    pub fn raw_oid(&self) -> &[u8] {
        &self.oid
    }

    pub fn oid(&self) -> &str {
        std::str::from_utf8(self.raw_oid()).expect("OID was not valid UTF-8")
    }

    pub fn raw_nspace(&self) -> &[u8] {
        &self.nspace
    }

    pub fn nspace(&self) -> &str {
        std::str::from_utf8(self.raw_nspace()).expect("Nspace was not valid UTF-8")
    }

    pub fn raw_locator(&self) -> &[u8] {
        &self.locator
    }

    pub fn locator(&self) -> &str {
        std::str::from_utf8(self.raw_locator()).expect("Locator was not valid UTF-8")
    }

    pub fn into_parts(self) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        (self.oid, self.locator, self.nspace)
    }
}

impl<'rados> IoCtx<'rados> {
    pub fn list_objects<'ioctx>(&'ioctx self) -> Result<List<'ioctx, 'rados>> {
        let mut list = std::ptr::null_mut();

        maybe_err(unsafe { rados_nobjects_list_open(self.inner(), &mut list) })?;

        Ok(List {
            inner: list,
            _phantom: Default::default(),
        })
    }
}

pub struct List<'ioctx, 'rados> {
    inner: rados_list_ctx_t,
    _phantom: PhantomData<&'ioctx IoCtx<'rados>>,
}

unsafe impl Send for List<'_, '_> {}
unsafe impl Sync for List<'_, '_> {}

impl<'ioctx, 'rados> List<'ioctx, 'rados> {
    pub fn get_pg_hash_position(&self) -> u32 {
        unsafe { rados_nobjects_list_get_pg_hash_position(self.inner) }
    }

    pub fn seek(&mut self, pos: u32) -> u32 {
        unsafe { rados_nobjects_list_seek(self.inner, pos) }
    }

    pub fn try_next(&mut self) -> Result<Option<Object<'_>>> {
        let mut oid = std::ptr::null();
        let mut oid_length = 0;

        let mut locator = std::ptr::null();
        let mut locator_length = 0;

        let mut nspace = std::ptr::null();
        let mut nspace_length = 0;

        let res = unsafe {
            rados_nobjects_list_next2(
                self.inner,
                &mut oid,
                &mut locator,
                &mut nspace,
                &mut oid_length,
                &mut locator_length,
                &mut nspace_length,
            )
        };

        if res == 0 {
            let object = rados_object_list_item {
                oid_length,
                oid: oid as _,
                nspace_length,
                nspace: nspace as _,
                locator_length,
                locator: locator as _,
            };

            Ok(Some(Object {
                value: object,
                _phantom: Default::default(),
            }))
        } else {
            let err = RadosError::from(res);

            if err == RadosError::Noent {
                Ok(None)
            } else {
                Err(err)
            }
        }
    }
}

impl Iterator for List<'_, '_> {
    type Item = OwnedObject;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.try_next().ok().flatten()?;

        Some(OwnedObject {
            oid: next.raw_oid().to_vec(),
            locator: next.raw_locator().to_vec(),
            nspace: next.raw_nspace().to_vec(),
        })
    }
}

impl Drop for List<'_, '_> {
    fn drop(&mut self) {
        unsafe { rados_nobjects_list_close(self.inner) };
    }
}
