//! A library implementing the [`msgr2`][0] protocol used by [Ceph][1].
//!
//! [0]: https://docs.ceph.com/en/quincy/dev/msgr2/
//! [1]: https://ceph.com/en/

mod ceph_features;
mod crypto;
mod entity_address;
mod entity_name;
mod entity_type;
pub mod frame;
mod key;
pub mod messages;

pub use ceph_features::CephFeatureSet;
pub use crypto::{decode_decrypt_enc_bl, encode_encrypt, encode_encrypt_enc_bl};
pub use entity_address::{EntityAddress, EntityAddressType};
pub use entity_name::EntityName;
pub use entity_type::EntityType;
pub use key::CryptoKey;

mod sealed {
    pub trait Sealed {}
}
