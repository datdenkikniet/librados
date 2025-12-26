use crate::{
    EncodeExt, EntityName,
    messages::auth::{AuthMethod, ConMode},
};

pub trait AuthRequestPayload: crate::sealed::Sealed {
    const METHOD: AuthMethod;

    fn payload(&self) -> Vec<u8>;
}

#[derive(Debug, Clone)]
pub struct AuthRequest {
    method: AuthMethod,
    preferred_modes: Vec<u32>,
    auth_payload: Vec<u8>,
}

impl AuthRequest {
    pub fn new<T>(auth_method: T, preferred_modes: Vec<ConMode>) -> Self
    where
        T: AuthRequestPayload,
    {
        let preferred_modes = preferred_modes
            .into_iter()
            .map(|v| u8::from(v) as u32)
            .collect();

        Self {
            method: T::METHOD,
            preferred_modes,
            auth_payload: auth_method.payload(),
        }
    }
}

impl EncodeExt for AuthRequest {
    fn encode(&self, buffer: &mut Vec<u8>) {
        (u8::from(self.method) as u32).encode(buffer);
        self.preferred_modes.encode(buffer);
        self.auth_payload.encode(buffer);
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
        self.name.encode(&mut buffer);
        self.global_id.encode(&mut buffer);

        buffer
    }
}
