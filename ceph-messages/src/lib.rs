mod config;
mod message;
mod mon_map;
mod mon_sub;
mod osd_map;

use ceph_foundation::DecodeError;

pub use config::Config;
pub use message::CephMessage;
pub use mon_map::MonMap;
pub use mon_sub::{MonSubscribe, MonSubscribeItem};
pub use osd_map::OsdMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Epoch(pub u32);

impl ceph_foundation::Encode for Epoch {
    fn encode(&self, buffer: &mut impl ceph_foundation::Encoder) {
        self.0.encode(buffer);
    }
}

impl ceph_foundation::Decode<'_> for Epoch {
    fn decode(buffer: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(Self(u32::decode(buffer)?))
    }
}

#[derive(Debug, Clone)]
pub enum DecodeMessageError {
    DecodeError(DecodeError),
    NotEnoughSegments { have: usize, need: usize },
    TooManySegments { have: usize, want: usize },
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
