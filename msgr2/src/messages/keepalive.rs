use crate::Timestamp;

#[derive(Debug, Clone)]
pub struct Keepalive {
    pub timestamp: Timestamp,
}

write_decode_encode!(Keepalive = timestamp);

#[derive(Debug, Clone)]
pub struct KeepaliveAck {
    pub timestamp: Timestamp,
}

write_decode_encode!(KeepaliveAck = timestamp);
