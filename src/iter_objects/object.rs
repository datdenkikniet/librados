use crate::librados::{rados_object_list_free, rados_object_list_item};

macro_rules! spec_impl {
    ($ty:ty) => {
        impl $ty {
            pub fn oid(&self) -> &str {
                Object::oid(self)
            }

            pub fn nspace(&self) -> &str {
                Object::nspace(self)
            }

            pub fn locator(&self) -> &str {
                Object::locator(self)
            }

            pub fn raw_oid(&self) -> &[u8] {
                Object::raw_oid(self)
            }

            pub fn raw_nspace(&self) -> &[u8] {
                Object::raw_nspace(self)
            }

            pub fn raw_locator(&self) -> &[u8] {
                Object::raw_locator(self)
            }
        }
    };
}

spec_impl!(RawObject);
spec_impl!(RefObject<'_>);
spec_impl!(OwnedObject);

/// A trait describing the information available
/// on a rados object.
pub trait Object {
    fn raw_oid(&self) -> &[u8];
    fn raw_nspace(&self) -> &[u8];
    fn raw_locator(&self) -> &[u8];

    fn oid(&self) -> &str {
        std::str::from_utf8(self.raw_oid()).expect("OID was not valid UTF-8")
    }

    fn nspace(&self) -> &str {
        std::str::from_utf8(self.raw_nspace()).expect("Nspace was not valid UTF-8")
    }

    fn locator(&self) -> &str {
        std::str::from_utf8(self.raw_locator()).expect("Locator was not valid UTF-8")
    }
}

/// Raw objects (which must be freed by librados).
#[repr(transparent)]
pub struct RawObject {
    value: rados_object_list_item,
}

impl RawObject {
    pub fn as_ref(&self) -> RefObject<'_> {
        self.into()
    }

    pub fn to_owned(&self) -> OwnedObject {
        self.into()
    }
}

impl Object for RawObject {
    fn raw_oid(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.value.oid as _, self.value.oid_length) }
    }

    fn raw_nspace(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.value.nspace as _, self.value.nspace_length) }
    }

    fn raw_locator(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.value.locator as _, self.value.locator_length) }
    }
}

impl core::ops::Drop for RawObject {
    fn drop(&mut self) {
        unsafe { rados_object_list_free(1, &mut self.value) };
    }
}

/// An object reference.
#[derive(Debug, Clone, Copy)]
pub struct RefObject<'a> {
    oid: &'a [u8],
    locator: &'a [u8],
    nspace: &'a [u8],
}

impl<'a> RefObject<'a> {
    pub(crate) fn new(oid: &'a [u8], locator: &'a [u8], nspace: &'a [u8]) -> Self {
        Self {
            oid,
            locator,
            nspace,
        }
    }
}

impl<'a> From<&'a RawObject> for RefObject<'a> {
    fn from(value: &'a RawObject) -> Self {
        value.as_ref()
    }
}

impl<'a> From<&'a OwnedObject> for RefObject<'a> {
    fn from(value: &'a OwnedObject) -> Self {
        Self {
            oid: value.raw_oid(),
            locator: value.raw_locator(),
            nspace: value.raw_nspace(),
        }
    }
}

impl Object for RefObject<'_> {
    fn raw_oid(&self) -> &[u8] {
        &self.oid
    }

    fn raw_nspace(&self) -> &[u8] {
        &self.nspace
    }

    fn raw_locator(&self) -> &[u8] {
        &self.locator
    }
}

#[derive(Debug, Clone)]
pub struct OwnedObject {
    oid: Vec<u8>,
    locator: Vec<u8>,
    nspace: Vec<u8>,
}

impl From<&RawObject> for OwnedObject {
    fn from(value: &RawObject) -> Self {
        Self::from_object(value)
    }
}

impl From<RefObject<'_>> for OwnedObject {
    fn from(value: RefObject<'_>) -> Self {
        Self::from_object(&value)
    }
}

impl OwnedObject {
    fn from_object<T: Object>(obj: &T) -> Self {
        Self {
            oid: obj.raw_oid().to_vec(),
            locator: obj.raw_locator().to_vec(),
            nspace: obj.raw_nspace().to_vec(),
        }
    }

    pub fn into_parts(self) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        (self.oid, self.locator, self.nspace)
    }
}

impl Object for OwnedObject {
    fn raw_oid(&self) -> &[u8] {
        &self.oid
    }

    fn raw_nspace(&self) -> &[u8] {
        &self.nspace
    }

    fn raw_locator(&self) -> &[u8] {
        &self.locator
    }
}
