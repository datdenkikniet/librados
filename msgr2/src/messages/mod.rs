pub mod auth;
mod banner;
mod client_ident;
mod hello;
mod ident_missing_features;
mod keepalive;
mod server_ident;

pub use banner::Banner;
pub use client_ident::ClientIdent;
pub use hello::Hello;
pub use ident_missing_features::IdentMissingFeatures;
pub use keepalive::{Keepalive, KeepaliveAck};
pub use server_ident::ServerIdent;

use crate::Encode;

const FEATURE_REVISION_21: u64 = 1 << 0;
const FEATURE_COMPRESSION: u64 = 1 << 1;

#[derive(Debug, Clone, Copy)]
pub struct MsgrFeatures(u64);

impl MsgrFeatures {
    pub const fn empty() -> Self {
        Self(0)
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

impl Encode for MsgrFeatures {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.0.encode(buffer);
    }
}
