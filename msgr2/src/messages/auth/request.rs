use crate::{
    EntityName,
    messages::auth::{AuthMethod, ConMode},
};

pub trait AuthRequestPayload: crate::sealed::Sealed {
    const METHOD: AuthMethod;

    fn payload(&self) -> Vec<u8>;
}

#[derive(Debug, Clone)]
pub struct AuthRequest {
    method: AuthMethod,
    preferred_modes: Vec<ConMode>,
    auth_payload: Vec<u8>,
}

impl AuthRequest {
    pub fn new<T>(auth_method: T, preferred_modes: Vec<ConMode>) -> Self
    where
        T: AuthRequestPayload,
    {
        Self {
            method: T::METHOD,
            preferred_modes,
            auth_payload: auth_method.payload(),
        }
    }

    pub fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&(u8::from(self.method) as u32).to_le_bytes());

        buffer.extend_from_slice(&(self.preferred_modes.len() as u32).to_le_bytes());

        for mode in &self.preferred_modes {
            buffer.extend_from_slice(&(u8::from(*mode) as u32).to_le_bytes());
        }

        buffer.extend_from_slice(&(self.auth_payload.len() as u32).to_le_bytes());
        buffer.extend_from_slice(&self.auth_payload);
    }
}

#[derive(Debug, Clone)]
pub struct AuthMethodNone {
    pub name: EntityName,
    pub global_id: u64,
}

impl crate::sealed::Sealed for AuthMethodNone {}
impl AuthRequestPayload for AuthMethodNone {
    const METHOD: AuthMethod = AuthMethod::None;

    fn payload(&self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(9);

        buffer.push(1u8);
        self.name.write_to(&mut buffer);
        buffer.extend_from_slice(&self.global_id.to_le_bytes());

        buffer
    }
}
