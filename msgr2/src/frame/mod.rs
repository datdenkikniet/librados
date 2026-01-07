mod epilogue;
mod frame;
mod preamble;

pub(crate) use frame::Frame;
pub(crate) use preamble::Preamble;

pub use preamble::Tag;

use crate::key::AES_GCM_SIG_SIZE;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrameFormat {
    Rev0Crc,
    Rev1Crc,
    Rev0Secure,
    Rev1Secure,
}

impl FrameFormat {
    pub fn has_crc(&self) -> bool {
        match self {
            FrameFormat::Rev0Crc => true,
            FrameFormat::Rev1Crc => true,
            FrameFormat::Rev0Secure => false,
            FrameFormat::Rev1Secure => false,
        }
    }

    pub fn start_rx_bytes(&self) -> usize {
        match self {
            FrameFormat::Rev0Crc => crate::frame::Preamble::SERIALIZED_SIZE,
            FrameFormat::Rev1Crc => crate::frame::Preamble::SERIALIZED_SIZE,
            FrameFormat::Rev0Secure => todo!(),
            FrameFormat::Rev1Secure => {
                Preamble::SERIALIZED_SIZE + Preamble::REV1_SECURE_INLINE_SIZE + AES_GCM_SIG_SIZE
            }
        }
    }
}

#[test]
fn valid_frame() {
    use crate::frame::preamble::Preamble;

    let frame_data = &[
        01, 01, 36, 00, 00, 00, 08, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
        00, 00, 00, 00, 00, 63, 189, 107, 06, 01, 01, 01, 01, 28, 00, 00, 00, 02, 00, 00, 00, 00,
        00, 00, 00, 16, 00, 00, 00, 02, 00, 221, 90, 10, 00, 01, 05, 00, 00, 00, 00, 00, 00, 00,
        00, 00, 105, 92, 102, 236, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
    ];

    let rev = FrameFormat::Rev0Crc;
    let (preamble, frame_data) = frame_data
        .split_first_chunk::<{ Preamble::SERIALIZED_SIZE }>()
        .unwrap();
    let mut preamble = Preamble::parse(preamble, rev).unwrap();

    Frame::decode(&mut preamble, frame_data).expect("Valid frame should be parseable");
}
