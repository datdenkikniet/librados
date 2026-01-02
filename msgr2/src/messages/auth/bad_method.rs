use crate::messages::auth::{AuthMethod, ConMode};

#[derive(Debug, Clone)]
pub struct AuthBadMethod {
    pub method: AuthMethod,
    pub result: u32,
    pub allowed_methods: Vec<AuthMethod>,
    pub allowed_modes: Vec<ConMode>,
}

write_decode_encode!(AuthBadMethod = method | result | allowed_methods | allowed_modes);
