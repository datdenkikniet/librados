#[derive(Debug, Clone)]
pub struct AuthReplyMore {
    pub payload: Vec<u8>,
}

write_decode_encode!(AuthReplyMore = payload);
