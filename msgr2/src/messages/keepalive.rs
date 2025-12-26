use crate::{EncodeExt, messages::Timestamp};

#[derive(Debug, Clone)]
pub struct Keepalive {
    pub timestamp: Timestamp,
}

impl EncodeExt for Keepalive {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.timestamp.encode(buffer);
    }
}

#[derive(Debug, Clone)]
pub struct KeepaliveAck {}
