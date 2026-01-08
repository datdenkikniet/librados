/// Payload for an additional reply message.
#[derive(Debug, Clone)]
pub struct AuthReplyMore {
    /// The payload.
    pub payload: Vec<u8>,
}

write_decode_encode!(AuthReplyMore = payload);
