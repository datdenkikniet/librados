mod done;
mod request;
mod signature;

pub use done::AuthDone;
pub use request::{AuthMethodNone, AuthRequest, AuthRequestPayload};
pub use signature::AuthSignature;

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
