//! Types relevant to messages exchanged during the authentication and
//! authorization phase of a connection.

mod bad_method;
mod done;
mod reply_more;
mod request;
mod signature;

pub use bad_method::AuthBadMethod;
pub use done::AuthDone;
pub use reply_more::AuthReplyMore;
pub use request::{
    AuthMethodCephX, AuthMethodNone, AuthRequest, AuthRequestMore, AuthRequestPayload,
};
pub use signature::AuthSignature;

use ceph_foundation::{Decode, DecodeError, Encode};

/// An authentication method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMethod {
    /// Unknown.
    Unknown = 0,
    /// No authentication.
    None = 1,
    /// [CephX] authentication.
    ///
    /// [CephX]: https://docs.ceph.com/en/latest/dev/cephx/
    CephX = 2,
    /// GSS authentication (unsupported).
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

/// A connection mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConMode {
    /// Checksum based connection. This type of connection
    /// only employs CRCs for detection of corruption of in-flight
    /// data, but does _not_ provide any security measures.
    Crc = 1,
    /// A secured connection. This type of connection encrypts
    /// and signs all transmitted data, which provides confidentiality
    /// and integrity guarantees.
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
            // This is, technically, its own value. However,
            // it makes no sense to represent it: it is unknown,
            // after all.
            // 0 => Self::Unknown,
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
