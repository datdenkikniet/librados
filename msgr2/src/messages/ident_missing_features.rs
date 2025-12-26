use crate::{EncodeExt, messages::MsgrFeatures};

#[derive(Debug, Clone)]
pub struct IdentMissingFeatures {
    pub features: MsgrFeatures,
}

impl EncodeExt for IdentMissingFeatures {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.features.encode(buffer);
    }
}

impl IdentMissingFeatures {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() != 8 {
            return None;
        }

        let features = MsgrFeatures(u64::from_le_bytes(data.try_into().unwrap()));

        Some(Self { features })
    }
}
