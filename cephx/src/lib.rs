//! CephX messages.

mod ticket;

use std::collections::HashSet;
pub use ticket::{Ticket, TicketsAndConnectionSecret};

use ceph_foundation::{
    Decode, DecodeError, Encode, Encoder, Timestamp,
    crypto::{Key, decode_decrypt_enc_bl, encode_encrypt},
    entity::{EntityName, EntityType},
};

/// A CephX ticket blob.
#[derive(Debug, Clone, Default)]
pub struct CephXTicketBlob {
    /// The ID of the secret described by this blob.
    pub secret_id: u64,
    /// A ticket blob. This blob generally consists of
    /// the encoded-and-encrypted version of a [`CephXServiceTicket`].
    pub blob: Vec<u8>,
}

ceph_foundation::write_decode_encode!(CephXTicketBlob = const version 1 as u8 | secret_id | blob);

/// A CephX service ticket.
#[derive(Debug)]
pub struct CephXServiceTicket {
    /// The session key used for this ticket.
    pub session_key: Key,
    /// The duration for which this ticket is valid.
    pub validity: Timestamp,
}

ceph_foundation::write_decode_encode!(CephXServiceTicket = const version 1 as u8 | session_key | validity);

/// Service ticket information.
#[derive(Debug)]
pub struct CephXServiceTicketInfo {
    /// The authentication ticket associated with this
    /// information.
    pub auth_ticket: AuthTicket,
    /// The session key used for this ticket.
    pub session_key: Key,
}

ceph_foundation::write_decode_encode!(CephXServiceTicketInfo = const version 1 as u8 | auth_ticket | session_key);

/// The type of a CephX message.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
#[expect(missing_docs)]
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

/// A CephX response header.
#[derive(Debug, Clone, PartialEq)]
pub struct CephXResponseHeader {
    /// The type of the message.
    pub ty: CephXMessageType,
    /// The status of the message (non-zero corresponds to an error).
    pub status: u32,
}

ceph_foundation::write_decode_encode!(CephXResponseHeader = ty as u16 | status);

/// A CephX message.
#[derive(Debug)]
pub struct CephXMessage {
    /// The type of the message.
    ty: CephXMessageType,
    /// The payload of the message, containing type-specific
    /// data.
    payload: Vec<u8>,
}

impl Encode for CephXMessage {
    fn encode(&self, buffer: &mut impl Encoder) {
        (self.ty as u16).encode(buffer);
        buffer.extend_from_slice(&self.payload);
    }
}

impl CephXMessage {
    /// Create a new CephX message, with the given `value`
    /// as its payload.
    pub fn new<T>(ty: CephXMessageType, value: T) -> Self
    where
        T: Encode,
    {
        Self {
            ty,
            payload: value.to_vec(),
        }
    }

    /// Get the type of the message.
    pub fn ty(&self) -> CephXMessageType {
        self.ty
    }

    /// Get the payload of the message.
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

/// A CephX authentication key.
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
    /// Compute a [`CephXAuthenticateKey`] based on the
    /// provided `server_challenge`, `client_challenge`, and
    /// `key`.
    ///
    /// This `key` is generally the key that can be found in
    /// a `ceph.keyring`, i.e. the shared secret between you
    /// and the server you are (attempting to) communicate with.
    pub fn compute(
        server_challenge: u64,
        client_challenge: u64,
        key: &Key,
    ) -> CephXAuthenticateKey {
        struct ChallengeBlob {
            server_challenge: u64,
            client_challenge: u64,
        }

        impl Encode for ChallengeBlob {
            fn encode(&self, buffer: &mut impl Encoder) {
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

/// A CephX authenticate message.
#[derive(Debug)]
pub struct CephXAuthenticate {
    /// The client challenge value.
    pub client_challenge: u64,
    /// The authentication key associated with this connection,
    /// computed using this message's `client_challenge`.
    pub key: CephXAuthenticateKey,
    /// An optional old ticket.
    pub old_ticket: CephXTicketBlob,
    /// Other keys to request.
    pub other_keys: HashSet<EntityType>,
}

ceph_foundation::write_decode_encode!(CephXAuthenticate = const version 3 as u8 | client_challenge | key as u64 | old_ticket | other_keys as EntitySet);

struct EntitySet {
    value: u32,
}

ceph_foundation::write_decode_encode!(EntitySet = value);

impl From<&HashSet<EntityType>> for EntitySet {
    fn from(value: &HashSet<EntityType>) -> Self {
        let value = value.iter().fold(0u32, |a, v| a | a | u32::from(v));
        Self { value }
    }
}

impl TryFrom<EntitySet> for HashSet<EntityType> {
    type Error = DecodeError;

    fn try_from(value: EntitySet) -> Result<Self, Self::Error> {
        let mut set = HashSet::with_capacity(8);

        let mut mask = 1u32;
        for _ in 0..32 {
            let masked = value.value & mask;
            mask <<= 1;

            let ty = EntityType::try_from(masked)?;
            set.insert(ty);
        }

        Ok(set)
    }
}

#[derive(Debug)]
pub struct AuthCapsInfo {
    pub allow_all: bool,
    pub caps: Vec<u8>,
}

ceph_foundation::write_decode_encode!(AuthCapsInfo = const version 1 as u8 | allow_all | caps);

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

ceph_foundation::write_decode_encode!(AuthTicket = const version 2 as u8 | name | global_id | const 0xFFFF_FFFF_FFFF_FFFFu64 as u64 | created | expires | caps | flags);

/// A potentially encrypted CephX ticket blob.
// TODO: zeroize
#[derive(Debug, Clone)]
pub enum MaybeEncryptedCephXTicketBlob {
    /// An unencrypted CephX ticket blob.
    Unencrypted(CephXTicketBlob),
    /// An encrypted CephX ticket blob.
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
    fn encode(&self, buffer: &mut impl Encoder) {
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

/// A CephX server challenge.
#[derive(Debug, Clone)]
pub struct CephXServerChallenge {
    /// The challenge value.
    pub challenge: u64,
}

ceph_foundation::write_decode_encode!(CephXServerChallenge = const version 1 as u8 | challenge);

/// A collection of authentication service ticket information.
#[derive(Debug)]
pub struct AuthServiceTicketReply {
    /// The service ticket reply containing the ticket for the auth
    /// service.
    ///
    // TODO: is this always a single ticket?
    service_ticket_reply: ServiceTicketReply,
    /// Is also: `cbl`
    ///
    /// This value is encrypted using the `session_secret` for the
    /// contacted service, as found in `info_list`.
    ///
    /// In the end, this is an encoded `Vec<u8>` whose data makes
    /// up an encoded and encrypted `Vec<u8>` (with length indicator
    /// and auth info). So, to decode it, you must first decode it as
    /// a `[u8]`, and then [`decode_decrypt_enc_bl`][0] that value.
    ///
    /// [0]: msgr2::decode_decrypt_enc_bl
    connection_secret: Vec<u8>,
    /// Extra data, containing additionally requested tickets.
    extra_service_tickets: ServiceTicketReply,
}

impl AuthServiceTicketReply {
    pub fn decrypt(mut self, master_key: &Key) -> Result<TicketsAndConnectionSecret, DecodeError> {
        let tickets = &mut self.service_ticket_reply.tickets;

        let Some(AuthServiceTicketInfo {
            ty: EntityType::Auth,
            encrypted_session_ticket,
            refresh_ticket,
        }) = tickets.get_mut(0)
        else {
            assert!(
                tickets.is_empty(),
                "Expected only a single ticket from Auth."
            );
            return Err(DecodeError::Custom(
                "Expected a single auth service ticket.".to_string(),
            ));
        };

        let auth_service_ticket: CephXServiceTicket =
            decode_decrypt_enc_bl(encrypted_session_ticket, master_key)?;

        let encrypted = self.connection_secret.as_mut_slice();
        let mut encrypted = ceph_foundation::decode_full_mut_slice(encrypted)?;
        let auth_service_secret: &[u8] =
            decode_decrypt_enc_bl(&mut encrypted, &auth_service_ticket.session_key)?;

        let mut out_tickets = Vec::new();

        for mut info in self.extra_service_tickets.tickets {
            let session_ticket: CephXServiceTicket = decode_decrypt_enc_bl(
                &mut info.encrypted_session_ticket,
                &auth_service_ticket.session_key,
            )?;

            let refresh_ticket = info.refresh_ticket.clone();

            out_tickets.push(Ticket {
                ty: info.ty,
                session_ticket,
                refresh_ticket,
            })
        }

        out_tickets.push(Ticket {
            session_ticket: auth_service_ticket,
            ty: EntityType::Auth,
            refresh_ticket: refresh_ticket.clone(),
        });

        Ok(TicketsAndConnectionSecret {
            tickets: out_tickets,
            connection_secret: auth_service_secret.to_vec(),
        })
    }
}

ceph_foundation::write_decode_encode!(
    AuthServiceTicketReply =
        service_ticket_reply | connection_secret | extra_service_tickets as Vec<u8>
);

#[derive(Debug)]
struct ServiceTicketReply {
    tickets: Vec<AuthServiceTicketInfo>,
}

ceph_foundation::write_decode_encode!(ServiceTicketReply = const version 1 as u8 | tickets);

impl From<&ServiceTicketReply> for Vec<u8> {
    fn from(value: &ServiceTicketReply) -> Self {
        value.to_vec()
    }
}

impl TryFrom<Vec<u8>> for ServiceTicketReply {
    type Error = DecodeError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Ok(Self {
                tickets: Vec::new(),
            })
        } else {
            Decode::decode(&mut value.as_ref())
        }
    }
}

/// Information about an auth service session.
#[derive(Debug)]
struct AuthServiceTicketInfo {
    /// The entity type for which this service ticket is
    /// valid.
    ///
    /// In the Ceph code base, this is called `service_id`.
    pub ty: EntityType,
    /// The encrypted session ticket associated with this auth
    /// ticket.
    ///
    /// The encryption is generally the key that can be found in
    /// a `ceph.keyring`, i.e. the shared secret between you
    /// and the server you are (attempting to) communicate with.
    pub encrypted_session_ticket: Vec<u8>,
    /// The refresh ticket for the auth service.
    pub refresh_ticket: MaybeEncryptedCephXTicketBlob,
}

ceph_foundation::write_decode_encode!(
    AuthServiceTicketInfo = ty as u32 | const version 1 as u8 | encrypted_session_ticket | refresh_ticket
);
