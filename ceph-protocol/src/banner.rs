const FEATURE_REVISION_21: u64 = 1 << 0;
const FEATURE_COMPRESSION: u64 = 1 << 1;

#[derive(Debug, Clone, Copy)]
pub struct Features(u64);

impl Features {
    pub const fn revision_21(&self) -> bool {
        self.0 & FEATURE_REVISION_21 == FEATURE_REVISION_21
    }

    // pub const fn set_revision_21(&mut self, revision_21: bool) {
    //     if !revision_21 {
    //         self.0 &= !FEATURE_REVISION_21;
    //     } else {
    //         self.0 |= FEATURE_REVISION_21;
    //     }
    // }

    pub const fn compression(&self) -> bool {
        self.0 & FEATURE_COMPRESSION == FEATURE_COMPRESSION
    }

    // pub fn set_compression(&mut self, compression: bool) {
    //     if !compression {
    //         self.0 &= !FEATURE_COMPRESSION
    //     } else {
    //         self.0 |= FEATURE_COMPRESSION
    //     }
    // }
}

#[derive(Clone, Copy, Debug)]
pub struct Banner {
    supported_features: Features,
    required_features: Features,
}

impl Default for Banner {
    fn default() -> Self {
        let features = Features(0x0);

        Self {
            supported_features: features,
            required_features: features,
        }
    }
}

const HEADER: &'static [u8] = b"ceph v2\n";

impl Banner {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let (header, data) = data
            .split_at_checked(10)
            .ok_or("Not enough data to contain banner & length")?;

        if &header[..8] != HEADER {
            return Err("Header is not correct".into());
        }

        let data_len = u16::from_le_bytes([header[8], header[9]]) as usize;

        if data.len() != data_len {
            return Err("data length mismatch".into());
        }

        let (supported_features, data) = data
            .split_first_chunk::<8>()
            .ok_or("8 bytes of supported feature data")?;

        let supported_features = Features(u64::from_le_bytes(*supported_features));

        let (required_features, data) = data
            .split_first_chunk::<8>()
            .ok_or("8 bytes of required feature data")?;

        let required_features = Features(u64::from_le_bytes(*required_features));

        if data.len() != 0 {
            return Err(format!("Unexpected {} leftover bytes", data.len()));
        }

        Ok(Self {
            required_features,
            supported_features,
        })
    }

    pub fn write<'a>(&self, output: &'a mut [u8]) -> Result<&'a [u8], String> {
        if output.len() < 26 {
            return Err(format!(
                "Require 26 bytes of space, but got output buffer of {} bytes",
                output.len()
            ));
        }

        output[..8].copy_from_slice(HEADER);
        output[8..10].copy_from_slice(&16u16.to_le_bytes());
        output[10..18].copy_from_slice(&self.supported_features.0.to_le_bytes());
        output[18..26].copy_from_slice(&self.required_features.0.to_le_bytes());

        Ok(&output[..26])
    }

    pub fn supported(&self) -> &Features {
        &self.supported_features
    }

    pub fn required(&self) -> &Features {
        &self.required_features
    }
}
