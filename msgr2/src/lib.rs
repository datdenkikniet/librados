mod ceph_features;
pub mod connection;
mod crypto;
mod encdec;
mod entity_address;
mod entity_name;
mod entity_type;
pub mod frame;
mod key;
pub mod messages;

pub use ceph_features::CephFeatureSet;
pub use encdec::{Decode, DecodeError, Encode, WireString};
pub use entity_address::{EntityAddress, EntityAddressType};
pub use entity_name::EntityName;
pub use entity_type::EntityType;
pub use key::CryptoKey;

mod sealed {
    pub trait Sealed {}
}

#[derive(Debug, Clone)]
pub struct Timestamp {
    pub tv_sec: u32,
    pub tv_nsec: u32,
}

write_decode_encode!(Timestamp = tv_sec | tv_nsec);
