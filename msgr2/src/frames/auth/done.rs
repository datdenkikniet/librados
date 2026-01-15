use crate::frames::auth::ConMode;

/// Authentication is done, and an authenticated
/// session has started.
///
/// After this message is received, the connection switches to
/// secure mode if that was negotiated.
#[derive(Debug, Clone, PartialEq)]
pub struct AuthDone {
    /// The assigned global ID.
    pub global_id: u64,
    /// The established connection.
    pub connection_mode: ConMode,
    /// The authentication payload.
    pub auth_payload: Vec<u8>,
}

ceph_foundation::write_decode_encode!(AuthDone = global_id | connection_mode | auth_payload);
