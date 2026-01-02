use crate::{
    Encode, EntityName,
    messages::auth::{AuthMethod, ConMode},
};

pub trait AuthRequestPayload: crate::sealed::Sealed + Encode {
    const METHOD: AuthMethod;
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
            auth_payload: auth_method.to_vec(),
        }
    }
}

write_decode_encode!(AuthRequest = method | preferred_modes | auth_payload);

#[derive(Debug, Clone)]
pub struct AuthMethodNone {
    pub name: EntityName,
    pub global_id: u64,
}

impl crate::sealed::Sealed for AuthMethodNone {}

write_decode_encode!(AuthMethodNone = const version 1 as u8 | name | global_id);

impl AuthRequestPayload for AuthMethodNone {
    const METHOD: AuthMethod = AuthMethod::None;
}

/// As encoded in `MonClient.cc` -> `MonConnection::get_auth_request`.
///
/// See: `Monitor::handle_auth_request`
#[derive(Debug)]
pub struct AuthMethodCephX {
    pub name: EntityName,
    pub global_id: u64,
}

impl crate::sealed::Sealed for AuthMethodCephX {}

write_decode_encode!(AuthMethodCephX = const version 10 as u8 | name | global_id);

impl AuthRequestPayload for AuthMethodCephX {
    const METHOD: AuthMethod = AuthMethod::CephX;
}

#[derive(Debug, Clone)]
pub struct AuthRequestMore {
    pub payload: Vec<u8>,
}

write_decode_encode!(AuthRequestMore = payload);
