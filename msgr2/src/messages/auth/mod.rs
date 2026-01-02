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
    AuthServiceTicketInfo, CephXAuthenticate, CephXAuthenticateKey, CephXMessage, CephXMessageType,
    CephXTicket,
};
pub use signature::AuthSignature;

use crate::Encode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMethod {
    Unknown = 0,
    None = 1,
    CephX = 2,
    Gss = 4,
}

impl From<AuthMethod> for u8 {
    fn from(value: AuthMethod) -> Self {
        value as _
    }
}

impl TryFrom<u32> for AuthMethod {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let value = u8::try_from(value).map_err(|_| ())?;
        Self::try_from(value)
    }
}

impl TryFrom<u8> for AuthMethod {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let res = match value {
            0 => Self::Unknown,
            1 => Self::None,
            2 => Self::CephX,
            3 => Self::Gss,
            _ => return Err(()),
        };

        Ok(res)
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
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let value = u8::try_from(value).map_err(|_| ())?;
        Self::try_from(value)
    }
}

impl TryFrom<u8> for ConMode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let res = match value {
            0 => Self::Unknown,
            1 => Self::Crc,
            2 => Self::Secure,
            _ => return Err(()),
        };

        Ok(res)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConModeU32(pub ConMode);

impl Encode for ConModeU32 {
    fn encode(&self, buffer: &mut Vec<u8>) {
        u32::from(self.0 as u8).encode(buffer);
    }
}
