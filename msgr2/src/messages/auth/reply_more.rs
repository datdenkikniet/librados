/// Payload for an additional reply message.
#[derive(Debug, Clone)]
pub struct AuthReplyMore {
    /// The payload.
    pub payload: Vec<u8>,
}

ceph_foundation::write_decode_encode!(AuthReplyMore = payload);
