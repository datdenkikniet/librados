use crate::frames::MsgrFeatures;

/// The initial connection banner.
#[derive(Clone, Copy, Debug, PartialEq)]
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

const HEADER: &[u8] = b"ceph v2\n";

impl Banner {
    /// The size (in bytes) of a serialized banner.
    pub const SERIALIZED_SIZE: usize = 26;

    /// Create a new banner.
    pub fn new(supported_features: MsgrFeatures, required_features: MsgrFeatures) -> Self {
        Self {
            supported_features,
            required_features,
        }
    }

    /// Parse a banner from the provided buffer.
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

    /// Convert this banner into its serialized form.
    pub fn to_bytes(&self) -> [u8; Self::SERIALIZED_SIZE] {
        let mut output = [0u8; Self::SERIALIZED_SIZE];
        output[..8].copy_from_slice(HEADER);
        output[8..10].copy_from_slice(&16u16.to_le_bytes());
        output[10..18].copy_from_slice(&self.supported_features.0.to_le_bytes());
        output[18..26].copy_from_slice(&self.required_features.0.to_le_bytes());
        output
    }

    /// Check whether the features supported by `self` make
    /// us compatible with the required features `other`.
    pub fn compatible(&self, other: &Self) -> bool {
        self.supported_features.0 | other.required_features.0 == self.supported_features.0
    }

    /// Get the set of supported `msgr2` features.
    pub fn supported(&self) -> &MsgrFeatures {
        &self.supported_features
    }

    /// Get the set of required `msgr2` features.
    pub fn required(&self) -> &MsgrFeatures {
        &self.required_features
    }
}
