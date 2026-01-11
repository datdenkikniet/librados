use ceph_foundation::{CephFeatureSet, entity::EntityAddress};

/// A client identification message.
#[derive(Clone, Debug)]
pub struct ClientIdent {
    /// The addresses at which the client is reachable.
    ///
    // NOTE: this is technically an `entity_addrvec_t`, but
    // this library does not aim to support ceph versions
    // older than jewel, so we need not support it.
    pub addresses: Vec<EntityAddress>,
    /// The target that the entity (perceives) itself to
    /// be connecting to.
    pub target: EntityAddress,
    /// The global ID of the client.
    pub gid: i64,
    /// The global sequence number of the entity.
    pub global_seq: u64,
    /// The features supported by the client.
    pub supported_features: CephFeatureSet,
    /// Features supported by the client.
    pub required_features: CephFeatureSet,
    /// Connection flags.
    pub flags: u64,
    /// A client cookie.
    pub cookie: u64,
}

ceph_foundation::write_decode_encode!(ClientIdent = /* version for entity_addrvec_t */ const version 2 as u8 | addresses | target | gid | global_seq | supported_features as u64 | required_features as u64 | flags | cookie);
