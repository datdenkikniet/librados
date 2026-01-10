//! A library implementing the [`msgr2`][0] protocol used by [Ceph][1].
//!
//! [0]: https://docs.ceph.com/en/quincy/dev/msgr2/
//! [1]: https://ceph.com/en/

mod ceph_features;
mod entity_address;
mod entity_name;
mod entity_type;
mod frame;
pub mod frames;

pub use ceph_features::CephFeatureSet;
pub use entity_address::{EntityAddress, EntityAddressType};
pub use entity_name::EntityName;
pub use entity_type::EntityType;
pub use frame::{Frame, FrameEncryption, FrameFormat, Revision, Tag, wire};

mod sealed {
    pub trait Sealed {}
}
