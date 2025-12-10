use std::{marker::PhantomData, u32};

use crate::{
    IoCtx, RadosError, RefObject, Result,
    error::maybe_err,
    iter_objects::OwnedObject,
    librados::{
        rados_list_ctx_t, rados_nobjects_list_close, rados_nobjects_list_get_pg_hash_position,
        rados_nobjects_list_next2, rados_nobjects_list_open,
    },
};

impl<'rados> IoCtx<'rados> {
    /// Create an iterator that will yield all objects in the pool and
    /// namespace configured on this [`IoCtx`].
    ///
    /// Note that filtering based on the namespace occurs synchronously,
    /// so yielding from the returned [`ObjectsIterator`] may take a long time.
    ///
    /// For more advanced use cases such as efficient subslicing of
    /// all objects, a [`Cursor`][0] can be used instead (see:
    /// [`IoCtx::object_cursor`]).
    ///
    /// [0]: super::Cursor
    pub fn objects<'ioctx>(&'ioctx self) -> Result<ObjectsIterator<'ioctx, 'rados>> {
        let mut list = std::ptr::null_mut();

        maybe_err(unsafe { rados_nobjects_list_open(self.inner(), &mut list) })?;

        Ok(ObjectsIterator {
            inner: list,
            _phantom: Default::default(),
        })
    }
}

/// An iterator yielding objects in a pool.
///
/// This struct is created by calling [`IoCtx::objects`].
///
/// Objects yielded by this cursor are synchronously filtered
/// by the namespace configured on the passed-in [`IoCtx`].
pub struct ObjectsIterator<'ioctx, 'rados> {
    inner: rados_list_ctx_t,
    _phantom: PhantomData<&'ioctx IoCtx<'rados>>,
}

unsafe impl Send for ObjectsIterator<'_, '_> {}
unsafe impl Sync for ObjectsIterator<'_, '_> {}

impl<'ioctx, 'rados> ObjectsIterator<'ioctx, 'rados> {
    /// Get the PG hash position for this iterator.
    ///
    /// See [`rados_nobjects_list_get_pg_hash_position`][0] for more
    /// information.
    ///
    /// [0]: https://docs.ceph.com/en/latest/rados/api/librados/#c.rados_nobjects_list_get_pg_hash_position
    pub fn get_pg_hash_position(&self) -> u32 {
        unsafe { rados_nobjects_list_get_pg_hash_position(self.inner) }
    }

    /// Try to get the next object from this iterator.
    ///
    /// See [`RefObject`] and the conversions it implements
    /// for more information.
    pub fn try_next(&mut self) -> Result<Option<RefObject<'_>>> {
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
            let oid = unsafe { core::slice::from_raw_parts(oid as _, oid_length) };
            let locator = unsafe { core::slice::from_raw_parts(locator as _, locator_length) };
            let nspace = unsafe { core::slice::from_raw_parts(nspace as _, nspace_length) };

            Ok(Some(RefObject::new(oid, locator, nspace)))
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

impl Iterator for ObjectsIterator<'_, '_> {
    type Item = Result<OwnedObject>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.try_next().map(|v| v.map(Into::into)).transpose()?;

        Some(next)
    }
}

impl Drop for ObjectsIterator<'_, '_> {
    fn drop(&mut self) {
        unsafe { rados_nobjects_list_close(self.inner) };
    }
}
