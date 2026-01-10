use crate::CephFeatureSet;

/// A message indicating that the transmitted `Ident`
/// does not support enough features to continue communication.
#[derive(Debug, Clone)]
pub struct IdentMissingFeatures {
    /// The missing features.
    pub features: CephFeatureSet,
}

ceph_foundation::write_decode_encode!(IdentMissingFeatures = features as u64);
