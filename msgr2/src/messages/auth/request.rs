use crate::{
    Encode, EntityName,
    messages::auth::{AuthMethod, ConMode, ConModeU32},
};

pub trait AuthRequestPayload: crate::sealed::Sealed + Encode {
    const METHOD: AuthMethod;
}

#[derive(Debug, Clone)]
pub struct AuthRequest {
    method: AuthMethod,
    preferred_modes: Vec<ConModeU32>,
    auth_payload: Vec<u8>,
}

impl AuthRequest {
    pub fn new<T>(auth_method: T, preferred_modes: Vec<ConMode>) -> Self
    where
        T: AuthRequestPayload,
    {
        let preferred_modes = preferred_modes.into_iter().map(ConModeU32).collect();

        Self {
            method: T::METHOD,
            preferred_modes,
            auth_payload: auth_method.to_vec(),
        }
    }

    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 12 {
            return Err(format!("Expected at least 12 bytes of auth request data"));
        }

        let method = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let Ok(method) = AuthMethod::try_from(method) else {
            return Err(format!("Unknown auth method {method}"));
        };

        let len = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        let mut preferred_modes = Vec::with_capacity(len as usize);
        let mut left = &data[8..];

        for _ in 0..len {
            let (val, new_left) = left
                .split_first_chunk::<4>()
                .ok_or("Ran out of data to construct preferred modes")?;

            let mode = u32::from_le_bytes(*val);
            let Ok(mode) = ConMode::try_from(mode) else {
                return Err(format!("Unknown connection mode {mode}"));
            };

            preferred_modes.push(ConModeU32(mode));
            left = new_left;
        }

        let (len, left) = left
            .split_first_chunk::<4>()
            .ok_or("Not enough data to construct auth payload")?;

        let len = u32::from_le_bytes(*len);

        if left.len() != len as usize {
            return Err(format!(
                "Expected {} bytes of auth payload, got {}",
                len,
                data.len()
            ));
        }

        let auth_payload = left.to_vec();

        Ok(Self {
            method,
            preferred_modes,
            auth_payload,
        })
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

/// As encoded in `MonClient.cc` -> `MonConnection::get_auth_request`.
///
/// See: `Monitor::handle_auth_request`
#[derive(Debug)]
pub struct AuthMethodCephX {
    pub name: EntityName,
    pub global_id: u64,
}

impl crate::sealed::Sealed for AuthMethodCephX {}

impl Encode for AuthMethodCephX {
    fn encode(&self, buffer: &mut Vec<u8>) {
        // Auth mode
        buffer.push(10);
        self.name.encode(buffer);
        self.global_id.encode(buffer);
    }
}

impl AuthRequestPayload for AuthMethodCephX {
    const METHOD: AuthMethod = AuthMethod::CephX;
}

#[derive(Debug, Clone)]
pub struct AuthRequestMore {
    pub payload: Vec<u8>,
}

impl Encode for AuthRequestMore {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.payload.encode(buffer);
    }
}
