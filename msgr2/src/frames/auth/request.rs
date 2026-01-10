use ceph_foundation::Encode;

use crate::{
    EntityName,
    frames::auth::{AuthMethod, ConMode},
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
    /// A prioritized list of preferred connection modes.
    preferred_modes: Vec<ConMode>,
    auth_payload: Vec<u8>,
}

impl AuthRequest {
    /// Create a new authentication request for the provided method and preferred methods.
    ///
    /// `preferred_methods` is an ordered list (where the first item is the most-preferred),
    /// from which the server that is being communicated with will pick a connection mode.
    /// See [`ConMode`] for more information about the different connection modes.
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

ceph_foundation::write_decode_encode!(AuthRequest = method | preferred_modes | auth_payload);

/// No authentication.
#[derive(Debug, Clone)]
pub struct AuthMethodNone {
    /// The name of the authenticating entity.
    pub name: EntityName,
    /// The requested global ID.
    pub global_id: u64,
}

impl crate::sealed::Sealed for AuthMethodNone {}

ceph_foundation::write_decode_encode!(AuthMethodNone = const version 1 as u8 | name | global_id);

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

ceph_foundation::write_decode_encode!(AuthMethodCephX = const version 10 as u8 | name | global_id);

impl AuthRequestPayload for AuthMethodCephX {
    const METHOD: AuthMethod = AuthMethod::CephX;
}

/// Additional data in an authentication request.
#[derive(Debug, Clone)]
pub struct AuthRequestMore {
    /// The payload of the message.
    pub payload: Vec<u8>,
}

ceph_foundation::write_decode_encode!(AuthRequestMore = payload);
