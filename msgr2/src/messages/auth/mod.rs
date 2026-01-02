mod bad_method;
mod done;
mod reply_more;
mod request;
mod signature;

pub use bad_method::AuthBadMethod;
pub use done::AuthDone;
pub use reply_more::{AuthReplyMore, CephXServerChallenge};
pub use request::{
    AuthMethodCephX, AuthMethodNone, AuthRequest, AuthRequestMore, AuthRequestPayload,
};
pub use signature::AuthSignature;

use crate::{Decode, DecodeError, Encode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMethod {
    Unknown = 0,
    None = 1,
    CephX = 2,
    Gss = 4,
}

impl From<AuthMethod> for u32 {
    fn from(value: AuthMethod) -> Self {
        value as u32
    }
}

impl TryFrom<u32> for AuthMethod {
    type Error = DecodeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let res = match value {
            0 => Self::Unknown,
            1 => Self::None,
            2 => Self::CephX,
            3 => Self::Gss,
            _ => {
                return Err(DecodeError::UnknownValue {
                    ty: "AuthMethod",
                    value: format!("{value}"),
                });
            }
        };

        Ok(res)
    }
}

impl Encode for AuthMethod {
    fn encode(&self, buffer: &mut Vec<u8>) {
        u32::from(*self).encode(buffer);
    }
}

impl Decode<'_> for AuthMethod {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, DecodeError> {
        let value = u32::decode(buffer)?;

        Self::try_from(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConMode {
    Unknown = 0,
    Crc = 1,
    Secure = 2,
}

impl From<ConMode> for u8 {
    fn from(value: ConMode) -> Self {
        value as _
    }
}

impl TryFrom<u32> for ConMode {
    type Error = DecodeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let value =
            u8::try_from(value).map_err(|_| DecodeError::unknown_value("ConMode", value))?;
        Self::try_from(value)
    }
}

impl TryFrom<u8> for ConMode {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let res = match value {
            0 => Self::Unknown,
            1 => Self::Crc,
            2 => Self::Secure,
            _ => return Err(DecodeError::unknown_value("ConMode", value)),
        };

        Ok(res)
    }
}

impl Encode for ConMode {
    fn encode(&self, buffer: &mut Vec<u8>) {
        u32::from(u8::from(*self)).encode(buffer);
    }
}

impl Decode<'_> for ConMode {
    fn decode(buffer: &mut &[u8]) -> Result<Self, DecodeError> {
        let value = u32::decode(buffer)?;
        Self::try_from(value)
    }
}
