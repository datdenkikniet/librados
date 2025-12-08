use crate::librados::{rados_object_list_free, rados_object_list_item};

mod sealed {
    pub(crate) trait Sealed {}
}

macro_rules! spec_impl {
    ($ty:ty) => {
        impl sealed::Sealed for $ty {}

        impl $ty {
            /// Get the OID of this object.
            pub fn oid(&self) -> &str {
                ListObject::oid(self)
            }

            /// Get the namespace of this object.
            pub fn nspace(&self) -> &str {
                ListObject::nspace(self)
            }

            /// Get the locator of this object.
            pub fn locator(&self) -> &str {
                ListObject::locator(self)
            }

            /// See [`ListObject::raw_oid`].
            pub fn raw_oid(&self) -> &[u8] {
                ListObject::raw_oid(self)
            }

            /// See [`ListObject::raw_nspace`].
            pub fn raw_nspace(&self) -> &[u8] {
                ListObject::raw_nspace(self)
            }

            /// See [`ListObject::raw_locator`].
            pub fn raw_locator(&self) -> &[u8] {
                ListObject::raw_locator(self)
            }
        }
    };
}

spec_impl!(RawObject);
spec_impl!(RefObject<'_>);
spec_impl!(OwnedObject);

/// A trait describing the information available
/// on a rados objects obtained from a list.
///
/// See [`IoCtx::objects`][0] and [`IoCtx::object_cursor`][1]
/// to obtain such objects.
///
/// [0]: crate::IoCtx::objects
/// [1]: crate::IoCtx::object_cursor
#[allow(private_bounds)]
pub trait ListObject: Send + Sync + sealed::Sealed {
    /// Get the raw bytes making up the OID of this
    /// object.
    ///
    /// Sometimes, the OID contains interior NUL or
    /// other invalid string bytes. In those cases,
    /// this function can be used to inspect the raw
    /// data.
    fn raw_oid(&self) -> &[u8];

    /// Get the raw bytes making up the namespace of this
    /// object.
    ///
    /// Sometimes, the namespace contains interior NUL or
    /// other invalid string bytes. In those cases,
    /// this function can be used to inspect the raw
    /// data.
    fn raw_nspace(&self) -> &[u8];

    /// Get the raw bytes making up the locator for this
    /// object.
    ///
    /// Sometimes, the locator contains interior NUL or
    /// other invalid string bytes. In those cases,
    /// this function can be used to inspect the raw
    /// data.
    fn raw_locator(&self) -> &[u8];

    /// Get the OID of this object.
    fn oid(&self) -> &str {
        std::str::from_utf8(self.raw_oid()).expect("OID was not valid UTF-8")
    }

    /// Get the namespace of this object.
    fn nspace(&self) -> &str {
        std::str::from_utf8(self.raw_nspace()).expect("Nspace was not valid UTF-8")
    }

    /// Get the locator of this object.
    fn locator(&self) -> &str {
        std::str::from_utf8(self.raw_locator()).expect("Locator was not valid UTF-8")
    }
}

/// Raw objects (that are freed by librados).
#[repr(transparent)]
pub struct RawObject {
    value: rados_object_list_item,
}

unsafe impl Send for RawObject {}
unsafe impl Sync for RawObject {}

impl RawObject {
    /// Convert this raw object into a [`RefObject`].
    ///
    /// [`RefObject`] also implements `From<&RawObject>`.
    pub fn as_ref(&self) -> RefObject<'_> {
        self.into()
    }

    /// Convert this raw object into an [`OwnedObject`].
    ///
    /// [`OwnedObject`] also implements `From<&RawObject>`.
    pub fn to_owned(&self) -> OwnedObject {
        self.into()
    }
}

impl ListObject for RawObject {
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
        // Free one invididual item at a time.
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

impl ListObject for RefObject<'_> {
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

/// An owned [`ListObject`].
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
    fn from_object<T: ListObject>(obj: &T) -> Self {
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

impl ListObject for OwnedObject {
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
