use crate::CephFeatureSet;

#[derive(Debug, Clone)]
pub struct IdentMissingFeatures {
    pub features: CephFeatureSet,
}

write_decode_encode!(IdentMissingFeatures = features as u64);
