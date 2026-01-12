use ceph_foundation::{Decode, DecodeError, Encode};

pub mod connection;

#[derive(Debug)]
pub struct CephMessageHeader2 {
    pub seq: u64,
    pub tid: u64,
    pub ty: CephMessageType,
    pub priority: u16,
    pub version: u16,
    pub data_pre_padding_len: u32,
    // TODO: automatically mask against PAGE_MASK
    pub data_off: u16,
    pub ack_seq: u64,
    pub flags: CephMessageHeader2Flags,
    pub compat_version: u16,
    pub reserved: u16,
}

ceph_foundation::write_decode_encode!(
    CephMessageHeader2 = seq
        | tid
        | ty as u16
        | priority
        | version
        | data_pre_padding_len
        | data_off
        | ack_seq
        | flags
        | compat_version
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
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.0)
    }
}

macro_rules! msg_type {
    ($($n:ident = $v:literal,)*)  => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(u16)]
        pub enum CephMessageType {
            $(
                $n = $v,
            )*
        }

        impl TryFrom<u16> for CephMessageType {
            type Error = DecodeError;

            fn try_from(value: u16) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $v => Ok(Self::$n),
                    )*
                    v => Err(DecodeError::unknown_value("CephMessageType", v)),
                }
            }
        }

    };
}

msg_type! {
    ShutDown = 1,
    Ping = 2,
    MonMap = 4,
    MonGetMap = 5,
    MonGetOsdMap = 6,
    MonMetadata = 7,
    StatFs = 13,
    StatFsReply = 14,
    MonSubscribe = 15,
    MonSubscribeAck = 16,
    Auth = 17,
    AuthReply = 18,
    MonGetVersion = 19,
    MonGetVersionReply = 20,
    GetPoolStats = 58,
    GetPoolStatsReply = 59,
}

impl From<&CephMessageType> for u16 {
    fn from(value: &CephMessageType) -> Self {
        *value as u16
    }
}
