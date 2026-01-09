use crate::{
    Encode, EntityName,
    messages::auth::{AuthMethod, ConMode},
};

/// An authentication request payload.
pub trait AuthRequestPayload: crate::sealed::Sealed + Encode {
    /// The authentication method for which this payload
    /// can be used.
    const METHOD: AuthMethod;
}

/// An authentication request.
#[derive(Debug, Clone)]
pub struct AuthRequest {
    method: AuthMethod,
    preferred_modes: Vec<ConMode>,
    auth_payload: Vec<u8>,
}

impl AuthRequest {
    /// Create a new authentication request for the provided method.
    pub fn new<T>(auth_method: T) -> Self
    where
        T: AuthRequestPayload,
    {
        Self {
            method: T::METHOD,
            // TODO: this order matters? Why?
            preferred_modes: vec![ConMode::Secure, ConMode::Crc],
            auth_payload: auth_method.to_vec(),
        }
    }
}

write_decode_encode!(AuthRequest = method | preferred_modes | auth_payload);

/// No authentication.
#[derive(Debug, Clone)]
pub struct AuthMethodNone {
    /// The name of the authenticating entity.
    pub name: EntityName,
    /// The requested global ID.
    pub global_id: u64,
}

impl crate::sealed::Sealed for AuthMethodNone {}

write_decode_encode!(AuthMethodNone = const version 1 as u8 | name | global_id);

impl AuthRequestPayload for AuthMethodNone {
    const METHOD: AuthMethod = AuthMethod::None;
}

/// The CephX authentication method.
// As encoded in `MonClient.cc` -> `MonConnection::get_auth_request`.
//
// See: `Monitor::handle_auth_request`
#[derive(Debug)]
pub struct AuthMethodCephX {
    /// The name of the authenticating entity.
    pub name: EntityName,
    /// The requested global ID.
    pub global_id: u64,
}

impl crate::sealed::Sealed for AuthMethodCephX {}

write_decode_encode!(AuthMethodCephX = const version 10 as u8 | name | global_id);

impl AuthRequestPayload for AuthMethodCephX {
    const METHOD: AuthMethod = AuthMethod::CephX;
}

/// Additional data in an authentication request.
#[derive(Debug, Clone)]
pub struct AuthRequestMore {
    /// The payload of the message.
    pub payload: Vec<u8>,
}

write_decode_encode!(AuthRequestMore = payload);
