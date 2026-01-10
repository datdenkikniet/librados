//! A library implementing the [`msgr2`][0] protocol used by [Ceph][1].
//!
//! [0]: https://docs.ceph.com/en/quincy/dev/msgr2/
//! [1]: https://ceph.com/en/

mod frame;
pub mod frames;

pub use frame::{Frame, FrameEncryption, FrameFormat, Revision, Tag, wire};

mod sealed {
    pub trait Sealed {}
}
