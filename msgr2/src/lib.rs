//! A library implementing the [`msgr2`][0] protocol used by [Ceph][1].
//!
//! [0]: https://docs.ceph.com/en/quincy/dev/msgr2/
//! [1]: https://ceph.com/en/

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

/// A UTC timestamp.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Timestamp {
    /// The amount of seconds since the UTC epoch.
    pub tv_sec: u32,

    /// The fractional, nanosecond amount since the UTC epoch.
    pub tv_nsec: u32,
}

write_decode_encode!(Timestamp = tv_sec | tv_nsec);
