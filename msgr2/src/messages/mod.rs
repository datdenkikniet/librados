pub mod auth;
mod banner;
mod client_ident;
mod hello;
mod keepalive;

pub use banner::Banner;
pub use client_ident::ClientIdent;
pub use hello::Hello;
pub use keepalive::{Keepalive, KeepaliveAck};

const FEATURE_REVISION_21: u64 = 1 << 0;
const FEATURE_COMPRESSION: u64 = 1 << 1;

#[derive(Debug, Clone, Copy)]
pub struct Features(u64);

impl Features {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn get(&self) -> u64 {
        self.0
    }

    pub const fn revision_21(&self) -> bool {
        self.0 & FEATURE_REVISION_21 == FEATURE_REVISION_21
    }

    pub const fn set_revision_21(&mut self, revision_21: bool) {
        if !revision_21 {
            self.0 &= !FEATURE_REVISION_21;
        } else {
            self.0 |= FEATURE_REVISION_21;
        }
    }

    pub const fn compression(&self) -> bool {
        self.0 & FEATURE_COMPRESSION == FEATURE_COMPRESSION
    }

    pub fn set_compression(&mut self, compression: bool) {
        if !compression {
            self.0 &= !FEATURE_COMPRESSION
        } else {
            self.0 |= FEATURE_COMPRESSION
        }
    }
}

#[derive(Debug, Clone)]
pub struct Timestamp {
    pub tv_sec: u32,
    pub tv_nsec: u32,
}

impl Timestamp {
    pub fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&self.tv_sec.to_le_bytes());
        buffer.extend_from_slice(&self.tv_nsec.to_le_bytes());
    }

    pub fn parse(buffer: &mut [u8]) -> Option<(Self, usize)> {
        if buffer.len() < 8 {
            return None;
        }

        let tv_sec = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        let tv_nsec = u32::from_le_bytes([buffer[4], buffer[5], buffer[6], buffer[7]]);

        Some((Self { tv_sec, tv_nsec }, 8))
    }
}
