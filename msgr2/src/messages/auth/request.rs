use crate::{
    Encode, EntityName, EntityType,
    messages::auth::{AuthMethod, ConMode},
};

pub trait AuthRequestPayload: crate::sealed::Sealed + Encode {
    const METHOD: AuthMethod;
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
            auth_payload: auth_method.to_vec(),
        }
    }
}

impl Encode for AuthRequest {
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

impl Encode for AuthMethodNone {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.push(1u8);
        self.name.encode(buffer);
        self.global_id.encode(buffer);
    }
}

impl AuthRequestPayload for AuthMethodNone {
    const METHOD: AuthMethod = AuthMethod::None;
}

// This data is what's decoded by `cephx_verify_authorizer`
#[derive(Debug)]
pub struct AuthMethodCephX {
    // TODO: this can be multiple?
    pub service_id: EntityType,
    pub global_id: u64,
    pub ticket: CephXTicket,
}

impl crate::sealed::Sealed for AuthMethodCephX {}

impl Encode for AuthMethodCephX {
    fn encode(&self, buffer: &mut Vec<u8>) {
        // Authorizer version
        buffer.push(1);

        self.global_id.encode(buffer);
        (u8::from(self.service_id) as u32).encode(buffer);
        self.ticket.encode(buffer);
    }
}

impl AuthRequestPayload for AuthMethodCephX {
    const METHOD: AuthMethod = AuthMethod::CephX;
}

#[derive(Debug)]
pub struct CephXTicket {
    pub secret_id: u64,
    pub blob: Vec<u8>,
}

impl Encode for CephXTicket {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.push(1u8);
        self.secret_id.encode(buffer);
        self.blob.encode(buffer);
    }
}
