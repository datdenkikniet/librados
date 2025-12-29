use crate::{Encode, Timestamp};

#[derive(Debug, Clone)]
pub struct Keepalive {
    pub timestamp: Timestamp,
}

impl Encode for Keepalive {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.timestamp.encode(buffer);
    }
}

#[derive(Debug, Clone)]
pub struct KeepaliveAck {
    pub timestamp: Timestamp,
}

impl KeepaliveAck {
    pub fn parse(data: &[u8]) -> Option<Self> {
        let (ts, _) = Timestamp::parse(&data)?;
        Some(Self { timestamp: ts })
    }
}
