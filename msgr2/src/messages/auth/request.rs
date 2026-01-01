use crate::{
    Encode, EntityName, Timestamp,
    cephx::CryptoKey,
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

#[derive(Debug)]
pub struct AuthCapsInfo {
    pub allow_all: bool,
    pub caps: Vec<u8>,
}

impl Encode for AuthCapsInfo {
    fn encode(&self, buffer: &mut Vec<u8>) {
        // Struct version
        buffer.push(1u8);
        buffer.push(self.allow_all as u8);
        self.caps.encode(buffer);
    }
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

#[derive(Debug)]
pub struct AuthTicket {
    pub name: EntityName,
    pub global_id: u64,
    pub created: Timestamp,
    pub expires: Timestamp,
    pub caps: AuthCapsInfo,
    // TODO: proper flags type.
    pub flags: u32,
}

impl Encode for AuthTicket {
    fn encode(&self, buffer: &mut Vec<u8>) {
        // Struct version
        buffer.push(2u8);

        self.name.encode(buffer);
        self.global_id.encode(buffer);

        // CEPH_AUTH_UID_DEFAULT
        u64::MAX.encode(buffer);

        self.created.encode(buffer);
        self.expires.encode(buffer);
        self.caps.encode(buffer);
        self.flags.encode(buffer);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum CephXMessageType {
    GetAuthSesssionKey = 0x0100,
    GetPrincipalSessionKey = 0x0200,
    GetRotatingKey = 0x0400,
}

#[derive(Debug)]
pub struct CephXMessage<T>
where
    T: Encode,
{
    ty: CephXMessageType,
    value: T,
}

impl<T> Encode for CephXMessage<T>
where
    T: Encode,
{
    fn encode(&self, buffer: &mut Vec<u8>) {
        (self.ty as u16).encode(buffer);
        self.value.encode(buffer);
    }
}

impl<T> CephXMessage<T>
where
    T: Encode,
{
    pub fn new(ty: CephXMessageType, value: T) -> Self {
        Self { ty, value }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CephXAuthenticateKey(u64);

impl CephXAuthenticateKey {
    pub fn compute(
        server_challenge: u64,
        client_challenge: u64,
        key: &CryptoKey,
    ) -> CephXAuthenticateKey {
        struct ChallengeBlob {
            server_challenge: u64,
            client_challenge: u64,
        }

        impl Encode for ChallengeBlob {
            fn encode(&self, buffer: &mut Vec<u8>) {
                self.server_challenge.encode(buffer);
                self.client_challenge.encode(buffer);
            }
        }

        let challenge_blob = ChallengeBlob {
            server_challenge,
            client_challenge,
        }
        .encode_encrypt(key);

        let (chunks, _rem) = challenge_blob.as_chunks::<8>();

        let mut k = 0;

        for chunk in chunks {
            let cur = u64::from_ne_bytes(*chunk);
            k ^= cur;
        }

        Self(k)
    }
}

#[derive(Debug)]
pub struct CephXAuthenticate {
    pub client_challenge: u64,
    pub key: CephXAuthenticateKey,
    pub old_ticket: CephXTicket,
    // TODO: enum
    pub other_keys: u32,
}

impl Encode for CephXAuthenticate {
    fn encode(&self, buffer: &mut Vec<u8>) {
        // Struct version
        buffer.push(3u8);

        self.client_challenge.encode(buffer);
        self.key.0.encode(buffer);
        self.old_ticket.encode(buffer);
        self.other_keys.encode(buffer);
    }
}
