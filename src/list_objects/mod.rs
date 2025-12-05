mod basic;
mod cursors;

use std::marker::PhantomData;

pub use basic::List;
pub use cursors::Cursor;

use crate::librados::rados_object_list_item;

#[repr(transparent)]
pub struct Object<'c> {
    value: rados_object_list_item,
    _phantom: PhantomData<&'c ()>,
}

impl Object<'_> {
    pub fn raw_oid(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.value.oid as _, self.value.oid_length) }
    }

    pub fn oid(&self) -> &str {
        std::str::from_utf8(self.raw_oid()).expect("OID was not valid UTF-8")
    }

    pub fn raw_nspace(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.value.nspace as _, self.value.nspace_length) }
    }

    pub fn nspace(&self) -> &str {
        std::str::from_utf8(self.raw_nspace()).expect("Nspace was not valid UTF-8")
    }

    pub fn raw_locator(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.value.locator as _, self.value.locator_length) }
    }

    pub fn locator(&self) -> &str {
        std::str::from_utf8(self.raw_locator()).expect("Locator was not valid UTF-8")
    }
}
