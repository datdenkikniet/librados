use crate::{CephFeatureSet, Encode};

#[derive(Debug, Clone)]
pub struct IdentMissingFeatures {
    pub features: CephFeatureSet,
}

impl Encode for IdentMissingFeatures {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.features.encode(buffer);
    }
}

impl IdentMissingFeatures {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() != 8 {
            return None;
        }

        let features = u64::from_le_bytes(data.try_into().unwrap());

        let features = CephFeatureSet {
            bits: features,
            mask: features,
        };

        Some(Self { features })
    }
}
