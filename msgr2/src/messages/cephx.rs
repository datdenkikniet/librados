use crate::{CryptoKey, Encode, EntityName, Timestamp, crypto::encode_encrypt};

#[derive(Debug)]
pub struct CephXTicketBlob {
    pub secret_id: u64,
    pub blob: Vec<u8>,
}

impl CephXTicketBlob {
    pub fn parse(data: &[u8]) -> Result<(Self, usize), String> {
        if data.len() < 5 {
            return Err(format!(
                "Expected at least 5 bytes of CephXTicket data, got only {}",
                data.len()
            ));
        }

        if data[0] != 1 {
            return Err(format!(
                "Expected version 1 for CephXTicket, got {}",
                data[0]
            ));
        }

        let Some((secret_id, blob)) = data[1..].split_first_chunk::<8>() else {
            return Err(format!(
                "Expected at least 8 bytes of secret ID data, got only {}",
                data[1..].len()
            ));
        };

        let secret_id = u64::from_le_bytes(*secret_id);

        let Some((blob_len, blob)) = data.split_first_chunk::<4>() else {
            return Err(format!(
                "Expected at least 4 bytes of blob len data, got only {}",
                blob.len()
            ));
        };

        let blob_len = u32::from_le_bytes(*blob_len) as usize;

        if blob.len() < blob_len {
            return Err(format!(
                "Expected at least {} bytes of blob data, got only {}",
                blob_len,
                blob.len()
            ));
        }

        let blob = blob.to_vec();

        Ok((Self { secret_id, blob }, 1 + 9 + 4 + blob_len))
    }
}

impl Encode for CephXTicketBlob {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.push(1u8);
        self.secret_id.encode(buffer);
        self.blob.encode(buffer);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum CephXMessageType {
    GetAuthSessionKey = 0x0100,
    GetPrincipalSessionKey = 0x0200,
    GetRotatingKey = 0x0400,
}

impl TryFrom<u16> for CephXMessageType {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let value = match value {
            0x0100 => Self::GetAuthSessionKey,
            0x0200 => Self::GetPrincipalSessionKey,
            0x0400 => Self::GetRotatingKey,
            _ => return Err(()),
        };

        Ok(value)
    }
}

#[derive(Debug, Clone)]
pub struct CephXResponseHeader {
    pub ty: CephXMessageType,
    pub status: u32,
}

impl CephXResponseHeader {
    pub fn parse(data: &[u8; 6]) -> Result<Self, String> {
        let ty = u16::from_le_bytes(data[..2].try_into().unwrap());
        let Ok(ty) = CephXMessageType::try_from(ty) else {
            return Err(format!("Unknown cephx message type {}", ty));
        };

        let status = u32::from_le_bytes(data[2..6].try_into().unwrap());

        Ok(Self { ty, status })
    }
}

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

    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let Some((header, payload)) = data.split_first_chunk::<6>() else {
            return Err(format!(
                "Need at least 6 bytes of data for CephXMessage, got only {}",
                data.len()
            ));
        };

        let header = CephXResponseHeader::parse(header)?;

        if header.status == 0 {
            Ok(Self {
                ty: header.ty,
                payload: payload.to_vec(),
            })
        } else {
            Err(format!("Error: {}", header.status))
        }
    }

    pub fn ty(&self) -> CephXMessageType {
        self.ty
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
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
