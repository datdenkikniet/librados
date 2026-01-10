/// An authentication signature.
#[derive(Debug, Clone)]
pub struct AuthSignature {
    /// The `sha256_hmac` hash of all of the application-layer data
    /// that was sent by this entity up to the point where
    /// the connection became secured, encrypted using the
    /// session key for the current session (obtained from a
    /// ticket).
    ///
    /// If the connection is not secured, it is zero.
    ///
    /// The application-layer data (usually) consists of the banner
    /// and all frames exchanged up until this signature message (e.g.
    /// [`Hello`](crate::messages::Hello), [`AuthRequest`](crate::messages::auth::AuthRequest),
    /// or [`AuthReplyMore`](crate::messages::auth::AuthReplyMore))
    pub sha256_hmac: [u8; 32],
}

ceph_foundation::write_decode_encode!(AuthSignature = sha256_hmac);
