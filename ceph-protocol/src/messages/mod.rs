mod banner;
mod client_ident;
mod hello;

pub use banner::Banner;
pub use client_ident::ClientIdent;
pub use hello::{EntityType, Hello};

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
