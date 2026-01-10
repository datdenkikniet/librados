use ceph_foundation::{Decode, DecodeError, Encode, entity::EntityType};
use cephx::MaybeEncryptedCephXTicketBlob;

/// A collection of authentication service ticket information.
#[derive(Debug)]
pub struct AuthServiceTicketReply {
    /// The service ticket reply containing the ticket for the auth
    /// service.
    pub service_ticket_reply: ServiceTicketReply,
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
    pub connection_secret: Vec<u8>,
    /// Extra data, containing additionally requested tickets.
    pub extra_service_tickets: ServiceTicketReply,
}

ceph_foundation::write_decode_encode!(
    AuthServiceTicketReply =
        service_ticket_reply | connection_secret | extra_service_tickets as Vec<u8>
);

#[derive(Debug)]
pub struct ServiceTicketReply {
    pub tickets: Vec<AuthServiceTicketInfo>,
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
        Decode::decode(&mut value.as_ref())
    }
}

/// Information about an auth service session.
#[derive(Debug)]
pub struct AuthServiceTicketInfo {
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
