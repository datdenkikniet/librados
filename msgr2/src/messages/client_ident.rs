use crate::{CephFeatureSet, entity_address::EntityAddress};

/// A client identification message.
#[derive(Clone, Debug)]
pub struct ClientIdent {
    /// The addresses at which the client is reachable.
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

write_decode_encode!(ClientIdent = const version 2 as u8 | addresses | target | gid | global_seq | supported_features as u64 | required_features as u64 | flags | cookie);
