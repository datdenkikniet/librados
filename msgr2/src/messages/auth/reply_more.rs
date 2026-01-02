#[derive(Debug, Clone)]
pub struct AuthReplyMore {
    pub payload: Vec<u8>,
}

write_decode_encode!(AuthReplyMore = payload);

#[derive(Debug, Clone)]
pub struct CephXServerChallenge {
    pub challenge: u64,
}

write_decode_encode!(CephXServerChallenge = const version 1 as u8 | challenge);
