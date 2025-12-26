use crate::messages::MsgrFeatures;

#[derive(Clone, Copy, Debug)]
pub struct Banner {
    supported_features: MsgrFeatures,
    required_features: MsgrFeatures,
}

impl Default for Banner {
    fn default() -> Self {
        let features = MsgrFeatures(0x0);

        Self {
            supported_features: features,
            required_features: features,
        }
    }
}

const HEADER: &'static [u8] = b"ceph v2\n";

impl Banner {
    pub const SERIALIZED_SIZE: usize = 26;

    pub fn new(supported_features: MsgrFeatures, required_features: MsgrFeatures) -> Self {
        Self {
            supported_features,
            required_features,
        }
    }

    pub fn parse(data: &[u8; Self::SERIALIZED_SIZE]) -> Result<Self, String> {
        let (header, data) = data.split_at(10);

        if &header[..8] != HEADER {
            return Err("Header is not correct".into());
        }

        let data_len = u16::from_le_bytes([header[8], header[9]]) as usize;

        if data.len() != data_len {
            return Err("data length mismatch".into());
        }

        let (supported_features, data) = data
            .split_first_chunk::<8>()
            .expect("8 bytes of supported feature data");

        let supported_features = MsgrFeatures(u64::from_le_bytes(*supported_features));

        let (required_features, _) = data
            .split_first_chunk::<8>()
            .expect("8 bytes of required feature data");

        let required_features = MsgrFeatures(u64::from_le_bytes(*required_features));

        Ok(Self {
            required_features,
            supported_features,
        })
    }

    pub fn write<'a>(&self, output: &'a mut [u8; Self::SERIALIZED_SIZE]) {
        output[..8].copy_from_slice(HEADER);
        output[8..10].copy_from_slice(&16u16.to_le_bytes());
        output[10..18].copy_from_slice(&self.supported_features.0.to_le_bytes());
        output[18..26].copy_from_slice(&self.required_features.0.to_le_bytes());
    }

    pub fn supported(&self) -> &MsgrFeatures {
        &self.supported_features
    }

    pub fn required(&self) -> &MsgrFeatures {
        &self.required_features
    }
}
