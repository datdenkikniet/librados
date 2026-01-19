use crate::{Decode, Encode, Encoder};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Uuid(pub [u8; 16]);

impl Encode for Uuid {
    fn encode(&self, buffer: &mut impl Encoder) {
        self.0.encode(buffer);
    }
}

impl Decode<'_> for Uuid {
    fn decode(buffer: &mut &[u8]) -> Result<Self, crate::DecodeError> {
        let value = <[u8; 16]>::decode(buffer)?;
        Ok(Self(value))
    }
}
