use crate::messages::auth::{AuthMethod, ConMode};

/// A message indicating that the requested authentication method
/// was no good.
#[derive(Debug, Clone)]
pub struct AuthBadMethod {
    /// The requested method.
    pub method: AuthMethod,
    /// The result (usually a linux error code, i.e. -EACCESS)
    pub result: u32,
    /// The allowed auth methods.
    pub allowed_methods: Vec<AuthMethod>,
    /// The allowed connection modes.
    pub allowed_modes: Vec<ConMode>,
}

write_decode_encode!(AuthBadMethod = method | result | allowed_methods | allowed_modes);
