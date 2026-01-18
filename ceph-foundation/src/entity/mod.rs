mod address;
mod name;
mod ty;

pub use address::{EntityAddress, EntityAddressType};
pub use name::EntityName;
pub use ty::EntityType;

use crate::DecodeError;

/// Equivalent of `entity_addrvec_t`
pub struct AddrVec {
    vec: Vec<EntityAddress>,
}

impl From<&Vec<EntityAddress>> for AddrVec {
    fn from(value: &Vec<EntityAddress>) -> Self {
        Self { vec: value.clone() }
    }
}

impl TryFrom<AddrVec> for Vec<EntityAddress> {
    type Error = DecodeError;

    fn try_from(value: AddrVec) -> Result<Self, Self::Error> {
        Ok(value.vec)
    }
}

write_decode_encode!(AddrVec = const version 2 as u8 | vec);
