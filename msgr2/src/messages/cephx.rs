use crate::{
    CryptoKey, Decode, DecodeError, Encode, EntityName, Timestamp, crypto::encode_encrypt,
};

write_encdec!(CephXTicketBlob = const version 1 as u8 | secret_id | blob);

#[derive(Debug)]
pub struct CephXTicketBlob {
    pub secret_id: u64,
    pub blob: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum CephXMessageType {
    GetAuthSessionKey = 0x0100,
    GetPrincipalSessionKey = 0x0200,
    GetRotatingKey = 0x0400,
}

impl From<&CephXMessageType> for u16 {
    fn from(value: &CephXMessageType) -> Self {
        *value as u16
    }
}

impl TryFrom<u16> for CephXMessageType {
    type Error = DecodeError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let value = match value {
            0x0100 => Self::GetAuthSessionKey,
            0x0200 => Self::GetPrincipalSessionKey,
            0x0400 => Self::GetRotatingKey,
            _ => {
                return Err(DecodeError::UnknownValue {
                    ty: "CephXMessageType",
                    value: format!("{value}"),
                });
            }
        };

        Ok(value)
    }
}

#[derive(Debug, Clone)]
pub struct CephXResponseHeader {
    pub ty: CephXMessageType,
    pub status: u32,
}

write_encdec!(CephXResponseHeader = ty as u16 | status);

#[derive(Debug)]
pub struct CephXMessage {
    ty: CephXMessageType,
    payload: Vec<u8>,
}

impl Encode for CephXMessage {
    fn encode(&self, buffer: &mut Vec<u8>) {
        (self.ty as u16).encode(buffer);
        buffer.extend_from_slice(&self.payload);
    }
}

impl CephXMessage {
    pub fn new<T>(ty: CephXMessageType, value: T) -> Self
    where
        T: Encode,
    {
        Self {
            ty,
            payload: value.to_vec(),
        }
    }

    pub fn ty(&self) -> CephXMessageType {
        self.ty
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

impl Decode<'_> for CephXMessage {
    fn decode(buffer: &mut &[u8]) -> Result<Self, DecodeError> {
        let header = CephXResponseHeader::decode(buffer)?;
        if header.status == 0 {
            Ok(Self {
                ty: header.ty,
                payload: buffer.to_vec(),
            })
        } else {
            Err(DecodeError::Custom(format!(
                "CephX error. Status: {}",
                header.status
            )))
        }
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
        };

        let challenge_blob = encode_encrypt(&challenge_blob, key);

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
    pub old_ticket: CephXTicketBlob,
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

#[derive(Debug)]
pub struct AuthServiceTicketInfo {}

impl AuthServiceTicketInfo {
    pub fn parse(data: &[u8]) -> Self {
        // Version
        assert_eq!(data[0], 1);

        let num = u32::from_le_bytes(data[1..5].try_into().unwrap());

        let mut left = &data[5..];
        for _ in 0..num {
            // Service ID
            let _service_id = u32::from_le_bytes(left[0..4].try_into().unwrap());

            // Version
            assert_eq!(left[4], 1);

            left = &left[5..];

            // Service ticket (encrypted with principal secret)
            let len = u32::from_le_bytes(left[0..4].try_into().unwrap()) as usize;
            left = &left[4 + len..];

            // (Potentially encrypted) service ticket blob
            let _has_encrypted_ticket = left[0] != 0;
            let len = u32::from_le_bytes(left[1..5].try_into().unwrap()) as usize;
            assert!(left.len() > len);
            left = &left[5 + len..];
        }

        // cbl
        let len = u32::from_le_bytes(left[0..4].try_into().unwrap()) as usize;
        let left = &left[4 + len..];

        // extra
        let len = u32::from_le_bytes(left[0..4].try_into().unwrap()) as usize;
        let left = &left[4 + len..];

        assert_eq!(left.len(), 0);

        Self {}
    }
}
