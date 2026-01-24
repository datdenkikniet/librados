use std::collections::HashMap;

use ceph_foundation::Decode;

use crate::DecodeMessage;

#[derive(Debug, Clone)]
pub struct Config {
    pub config: HashMap<String, String>,
}

impl DecodeMessage<'_> for Config {
    fn decode_message(segments: &[&[u8]]) -> Result<Self, crate::DecodeMessageError> {
        if segments.len() > 1 {
            return Err(crate::DecodeMessageError::TooManySegments {
                have: segments.len(),
                want: 1,
            });
        } else if segments.is_empty() {
            return Err(crate::DecodeMessageError::NotEnoughSegments { have: 0, need: 1 });
        }

        let config = Decode::decode(&mut segments[0].as_ref())?;

        Ok(Self { config })
    }
}
