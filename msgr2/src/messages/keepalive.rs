use crate::Timestamp;

/// A keepalive message.
#[derive(Debug, Clone)]
pub struct Keepalive {
    /// The timestamp at which this message was sent.
    pub timestamp: Timestamp,
}

write_decode_encode!(Keepalive = timestamp);

/// A keepalive ack.
#[derive(Debug, Clone)]
pub struct KeepaliveAck {
    /// The timestamp of the received keepalive.
    pub timestamp: Timestamp,
}

write_decode_encode!(KeepaliveAck = timestamp);
