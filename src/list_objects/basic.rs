use std::marker::PhantomData;

use crate::{
    IoCtx, RadosError, Result,
    error::maybe_err,
    librados::{
        rados_list_ctx_t, rados_nobjects_list_close, rados_nobjects_list_get_pg_hash_position,
        rados_nobjects_list_next2, rados_nobjects_list_open, rados_nobjects_list_seek,
    },
};

impl<'rados> IoCtx<'rados> {
    pub fn list_objects<'ioctx>(&'ioctx self) -> Result<BasicList<'ioctx, 'rados>> {
        let mut list = std::ptr::null_mut();

        maybe_err(unsafe { rados_nobjects_list_open(self.inner(), &mut list) })?;

        Ok(BasicList {
            inner: list,
            _phantom: Default::default(),
        })
    }
}

pub struct Entry<T> {
    pub entry: T,
    pub key: T,
    pub nspace: T,
}

pub struct BasicList<'ioctx, 'rados> {
    inner: rados_list_ctx_t,
    _phantom: PhantomData<&'ioctx IoCtx<'rados>>,
}

unsafe impl Send for BasicList<'_, '_> {}
unsafe impl Sync for BasicList<'_, '_> {}

impl<'ioctx, 'rados> BasicList<'ioctx, 'rados> {
    pub fn get_pg_hash_position(&self) -> u32 {
        unsafe { rados_nobjects_list_get_pg_hash_position(self.inner) }
    }

    pub fn seek(&mut self, pos: u32) -> u32 {
        unsafe { rados_nobjects_list_seek(self.inner, pos) }
    }

    pub fn try_next(&mut self) -> Result<Option<Entry<&str>>> {
        let Entry { entry, key, nspace } = match self.try_next2()? {
            Some(v) => v,
            None => return Ok(None),
        };

        let entry = std::str::from_utf8(entry).expect("Entry was not a valid UTF-8 string");
        let key = std::str::from_utf8(key).expect("Entry was not a valid UTF-8 string");
        let nspace = std::str::from_utf8(nspace).expect("Entry was not a valid UTF-8 string");

        Ok(Some(Entry { entry, key, nspace }))
    }

    pub fn try_next2(&mut self) -> Result<Option<Entry<&[u8]>>> {
        let mut entry = std::ptr::null();
        let mut entry_len = 0;

        let mut key = std::ptr::null();
        let mut key_len = 0;

        let mut nspace = std::ptr::null();
        let mut nspace_len = 0;

        let res = unsafe {
            rados_nobjects_list_next2(
                self.inner,
                &mut entry,
                &mut key,
                &mut nspace,
                &mut entry_len,
                &mut key_len,
                &mut nspace_len,
            )
        };

        if res == 0 {
            use core::slice::from_raw_parts;

            let entry = unsafe { from_raw_parts(entry as *const u8, entry_len as _) };
            let key = unsafe { from_raw_parts(key as *const u8, key_len as _) };
            let nspace = unsafe { from_raw_parts(nspace as *const u8, nspace_len as _) };

            Ok(Some(Entry { entry, key, nspace }))
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

impl Iterator for BasicList<'_, '_> {
    type Item = Entry<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.try_next().ok().flatten()?;

        Some(Entry {
            entry: next.entry.into(),
            key: next.key.to_string(),
            nspace: next.nspace.to_string(),
        })
    }
}

impl Drop for BasicList<'_, '_> {
    fn drop(&mut self) {
        unsafe { rados_nobjects_list_close(self.inner) };
    }
}
