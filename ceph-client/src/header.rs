use std::num::NonZeroU16;

use ceph_foundation::{Decode, DecodeError, Encode, Encoder, write_decode_encode};

struct SomeOrZero {
    value: u16,
}

write_decode_encode!(SomeOrZero = value);

impl From<&Option<NonZeroU16>> for SomeOrZero {
    fn from(value: &Option<NonZeroU16>) -> Self {
        Self {
            value: value.map(|v| v.get()).unwrap_or(0),
        }
    }
}

impl TryFrom<SomeOrZero> for Option<NonZeroU16> {
    type Error = DecodeError;

    fn try_from(value: SomeOrZero) -> Result<Self, Self::Error> {
        Ok(NonZeroU16::new(value.value))
    }
}

#[derive(Debug)]
pub struct CephMessageHeader2 {
    pub seq: u64,
    pub transaction_id: u64,
    pub ty: u16,
    /// The priority of this message. Higher value = more important.
    pub priority: u16,
    /// The version of message encoding.
    pub version: u16,
    pub data_pre_padding_len: u32,
    // TODO: automatically mask against PAGE_MASK
    pub data_off: u16,
    pub ack_seq: u64,
    pub flags: CephMessageHeader2Flags,
    /// Code that can decode this version of the message
    /// (if known) that should be able to decode this message,
    /// even if it cannot fully decode this message at version
    /// `version`.
    pub compat_version: Option<NonZeroU16>,
    pub reserved: u16,
}

ceph_foundation::write_decode_encode!(
    CephMessageHeader2 = seq
        | transaction_id
        | ty
        | priority
        | version
        | data_pre_padding_len
        | data_off
        | ack_seq
        | flags
        | compat_version as SomeOrZero
        | reserved
);

#[derive(Debug)]
pub struct CephMessageHeader2Flags(pub u8);

impl Decode<'_> for CephMessageHeader2Flags {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, DecodeError> {
        let (value, rest) = buffer
            .split_first()
            .ok_or_else(|| DecodeError::NotEnoughData {
                field: None,
                have: 0,
                need: 1,
            })?;

        *buffer = rest;
        Ok(Self(*value))
    }
}

impl Encode for CephMessageHeader2Flags {
    fn encode(&self, buffer: &mut impl Encoder) {
        buffer.push(self.0)
    }
}
