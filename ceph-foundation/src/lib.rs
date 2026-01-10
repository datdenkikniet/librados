pub mod crypto;
mod encdec;
pub mod entity;
mod features;

pub use encdec::{Decode, DecodeError, Encode, WireString};
pub use features::CephFeatureSet;

/// A UTC timestamp.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Timestamp {
    /// The amount of seconds since the UTC epoch.
    pub tv_sec: u32,

    /// The fractional, nanosecond amount since the UTC epoch.
    pub tv_nsec: u32,
}

write_decode_encode!(Timestamp = tv_sec | tv_nsec);
