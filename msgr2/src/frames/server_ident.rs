use crate::{CephFeatureSet, EntityAddress};

/// A server identification message.
#[derive(Debug, Clone)]
pub struct ServerIdent {
    /// The entity addresses on which the server is reachable.
    pub addresses: Vec<EntityAddress>,
    /// The global ID of this entity. Combine this with its [`EntityType`][crate::EntityType]
    /// to obtain its name (i.e. `mon.0`).
    pub gid: i64,
    /// The global sequence number of the current connection.
    pub global_seq: u64,
    /// The features supported by the server.
    pub supported_features: CephFeatureSet,
    /// The features required by the server.
    pub required_features: CephFeatureSet,
    /// Connection flags.
    pub flags: u64,
    /// The connection cookie.
    pub cookie: u64,
}

ceph_foundation::write_decode_encode!(ServerIdent = const version 2 as u8 | addresses | gid | global_seq | supported_features as u64 | required_features as u64 | flags | cookie);
