mod message;
mod mon_map;

use ceph_foundation::DecodeError;

pub use message::CephMessage;
pub use mon_map::MonMap;

#[derive(Debug, Clone)]
pub enum DecodeMessageError {
    DecodeError(DecodeError),
    NotEnoughSegments { have: usize, need: usize },
    Custom(String),
}

impl From<DecodeError> for DecodeMessageError {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError(value)
    }
}

pub trait DecodeMessage<'a>: Sized {
    fn decode_message(segments: &[&'a [u8]]) -> Result<Self, DecodeMessageError>;
}

pub trait EncodeMessage<'a>: Sized {
    fn encode_message(output_segments: &mut Vec<Vec<u8>>);
}
