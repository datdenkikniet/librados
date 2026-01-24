mod config;
mod message;
mod mon_map;
mod mon_sub;

use ceph_foundation::DecodeError;

pub use config::Config;
pub use message::CephMessage;
pub use mon_map::MonMap;
pub use mon_sub::{MonSubscribe, MonSubscribeItem};

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
