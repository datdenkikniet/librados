mod ceph_features;
pub mod connection;
mod encode;
mod entity_address;
mod entity_name;
mod entity_type;
pub mod frame;
mod key;
pub mod messages;
mod crypto;

pub use ceph_features::CephFeatureSet;
pub use encode::Encode;
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

impl Encode for Timestamp {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.tv_sec.encode(buffer);
        self.tv_nsec.encode(buffer);
    }
}

impl Timestamp {
    pub fn parse(buffer: &[u8]) -> Option<(Self, usize)> {
        if buffer.len() < 8 {
            return None;
        }

        let tv_sec = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        let tv_nsec = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);

        Some((Self { tv_sec, tv_nsec }, 8))
    }
}
