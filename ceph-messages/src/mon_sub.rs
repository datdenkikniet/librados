use std::collections::HashMap;

use ceph_foundation::{Decode, Encode, write_decode_encode};

use crate::{DecodeMessage, DecodeMessageError};

#[derive(Debug, Clone)]
pub struct MonSubscribe {
    pub hostname: String,
    pub what: HashMap<String, MonSubscribeItem>,
}

impl DecodeMessage<'_> for MonSubscribe {
    fn decode_message(segments: &[&[u8]]) -> Result<Self, DecodeMessageError> {
        if segments.len() > 1 {
            return Err(DecodeMessageError::TooManySegments {
                have: segments.len(),
                want: 1,
            });
        }

        Self::decode(&mut segments[0].as_ref()).map_err(Into::into)
    }
}

write_decode_encode!(MonSubscribe = what | hostname);

#[derive(Debug, Clone)]
pub struct MonSubscribeItem {
    pub start: u64,
    pub flags: u8,
}

impl Encode for MonSubscribeItem {
    fn encode(&self, buffer: &mut impl ceph_foundation::Encoder) {
        self.start.encode(buffer);
        buffer.push(self.flags);
    }
}

impl Decode<'_> for MonSubscribeItem {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, ceph_foundation::DecodeError> {
        let start = u64::decode(buffer)?;
        let [flags]: [u8; 1] = Decode::decode(buffer)?;

        Ok(Self { start, flags })
    }
}
