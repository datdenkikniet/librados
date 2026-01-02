use crate::{CephFeatureSet, entity_address::EntityAddress};

#[derive(Clone, Debug)]
pub struct ClientIdent {
    pub addresses: Vec<EntityAddress>,
    pub target: EntityAddress,
    pub gid: i64,
    pub global_seq: u64,
    pub supported_features: CephFeatureSet,
    pub required_features: CephFeatureSet,
    pub flags: u64,
    pub cookie: u64,
}

write_decode_encode!(ClientIdent = const version 2 as u8 | addresses | target | gid | global_seq | supported_features as u64 | required_features as u64 | flags | cookie);
