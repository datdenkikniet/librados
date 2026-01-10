//! Messages that can be exchanged over a `msgr2` connection.

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

use ceph_foundation::{Decode, DecodeError, Encode};

const FEATURE_REVISION_21: u64 = 1 << 0;
const FEATURE_COMPRESSION: u64 = 1 << 1;

/// A set of `msgr2` features.
#[derive(Debug, Clone, Copy)]
pub struct MsgrFeatures(u64);

impl MsgrFeatures {
    /// The empty set.
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Whether this feature set indicates revision 2.1 support.
    pub const fn revision_21(&self) -> bool {
        self.0 & FEATURE_REVISION_21 == FEATURE_REVISION_21
    }

    /// Set whether this feature set indicates support for revision 2.1
    pub const fn set_revision_21(&mut self, revision_21: bool) {
        if !revision_21 {
            self.0 &= !FEATURE_REVISION_21;
        } else {
            self.0 |= FEATURE_REVISION_21;
        }
    }

    /// Whether this feature set indicates support for compression.
    pub const fn compression(&self) -> bool {
        self.0 & FEATURE_COMPRESSION == FEATURE_COMPRESSION
    }
}

impl Encode for MsgrFeatures {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.0.encode(buffer);
    }
}

impl Decode<'_> for MsgrFeatures {
    fn decode(buffer: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(Self(u64::decode(buffer)?))
    }
}
