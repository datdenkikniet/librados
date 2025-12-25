use crate::messages::Timestamp;

#[derive(Debug, Clone)]
pub struct Keepalive {
    pub timestamp: Timestamp,
}

impl Keepalive {
    pub fn write_to(&self, buffer: &mut Vec<u8>) {
        self.timestamp.write_to(buffer);
    }
}

#[derive(Debug, Clone)]
pub struct KeepaliveAck {}
