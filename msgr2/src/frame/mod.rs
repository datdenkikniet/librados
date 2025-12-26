mod epilogue;
mod frame;
mod preamble;

pub use frame::Frame;
pub use preamble::{Preamble, Tag};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Msgr2Revision {
    V2_0,
    V2_1,
}

#[test]
fn valid_frame() {
    let frame_data = &[
        01, 01, 36, 00, 00, 00, 08, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
        00, 00, 00, 00, 00, 63, 189, 107, 06, 01, 01, 01, 01, 28, 00, 00, 00, 02, 00, 00, 00, 00,
        00, 00, 00, 16, 00, 00, 00, 02, 00, 221, 90, 10, 00, 01, 05, 00, 00, 00, 00, 00, 00, 00,
        00, 00, 105, 92, 102, 236, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
    ];

    let rev = Msgr2Revision::V2_0;
    let (preamble, frame_data) = frame_data.split_at(Preamble::SERIALIZED_SIZE);
    let preamble = Preamble::parse(preamble, rev).unwrap();

    Frame::parse(&preamble, frame_data).expect("Valid frame should be parseable");
}
