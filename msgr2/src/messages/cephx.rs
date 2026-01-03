use crate::{
    CryptoKey, Decode, DecodeError, Encode, EntityName, EntityType, Timestamp,
    crypto::encode_encrypt,
};

#[derive(Debug)]
pub struct CephXTicketBlob {
    pub secret_id: u64,
    pub blob: Vec<u8>,
}

write_decode_encode!(CephXTicketBlob = const version 1 as u8 | secret_id | blob);

#[derive(Debug)]
pub struct CephXServiceTicket {
    pub session_key: CryptoKey,
    pub validity: Timestamp,
}

write_decode_encode!(CephXServiceTicket = const version 1 as u8 | session_key | validity);

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
                return Err(DecodeError::unknown_value("CephXMessageType", value));
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

write_decode_encode!(CephXResponseHeader = ty as u16 | status);

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

impl From<&CephXAuthenticateKey> for u64 {
    fn from(value: &CephXAuthenticateKey) -> Self {
        value.0
    }
}

impl TryFrom<u64> for CephXAuthenticateKey {
    type Error = DecodeError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}

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

write_decode_encode!(CephXAuthenticate = const version 3 as u8 | client_challenge | key as u64 | old_ticket | other_keys);

#[derive(Debug)]
pub struct AuthCapsInfo {
    pub allow_all: bool,
    pub caps: Vec<u8>,
}

write_decode_encode!(AuthCapsInfo = const version 1 as u8 | allow_all | caps);

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

write_decode_encode!(AuthTicket = const version 2 as u8 | name | global_id | const 0xFFFF_FFFF_FFFF_FFFFu64 as u64 | created | expires | caps | flags);

#[derive(Debug)]
pub struct AuthServiceTicketInfos {
    pub info_list: Vec<AuthServiceTicketInfo>,
    pub cbl: Vec<u8>,
    pub extra: Vec<u8>,
}

write_decode_encode!(AuthServiceTicketInfos = const version 1 as u8 | info_list | cbl | extra);

#[derive(Debug)]
pub struct AuthServiceTicketInfo {
    pub service_id: EntityType,
    pub encrypted_service_ticket: Vec<u8>,
    pub maybe_encrypted_blob: MaybeEncryptedCephXTicketBlob,
}

write_decode_encode!(
    AuthServiceTicketInfo = service_id as u32 | const version 1 as u8 | encrypted_service_ticket | maybe_encrypted_blob
);

#[derive(Debug)]
pub enum MaybeEncryptedCephXTicketBlob {
    Unencrypted(CephXTicketBlob),
    Encrypted(Vec<u8>),
}

impl Decode<'_> for MaybeEncryptedCephXTicketBlob {
    fn decode(buffer: &mut &[u8]) -> Result<Self, DecodeError> {
        let encrypted = bool::decode(buffer)?;

        let blob = Vec::decode(buffer)?;

        if encrypted {
            Ok(Self::Encrypted(blob))
        } else {
            let unencrypted_blob = CephXTicketBlob::decode(&mut blob.as_slice())?;
            Ok(Self::Unencrypted(unencrypted_blob))
        }
    }
}

impl Encode for MaybeEncryptedCephXTicketBlob {
    fn encode(&self, buffer: &mut Vec<u8>) {
        match self {
            MaybeEncryptedCephXTicketBlob::Unencrypted(ceph_xticket_blob) => {
                false.encode(buffer);
                let mut blob_vec = Vec::new();
                ceph_xticket_blob.encode(&mut blob_vec);
                blob_vec.encode(buffer);
            }
            MaybeEncryptedCephXTicketBlob::Encrypted(items) => {
                true.encode(buffer);
                items.encode(buffer);
            }
        }
    }
}
