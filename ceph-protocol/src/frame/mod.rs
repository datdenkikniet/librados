mod epilogue;
mod frame;
mod preamble;

pub use frame::Frame;
pub use preamble::Tag;

/// The algorithm parameters used for the CRC
/// calculated by Ceph.
///
/// Note: these parameters do _not_ match the `crc32-c` (CASTAGNOLI)
/// algorithm.
const ALGO: crc::Algorithm<u32> = crc::Algorithm {
    width: 32,
    poly: 0x1EDC6F41,
    init: 0,
    refin: true,
    refout: true,
    xorout: 0,
    check: 0,
    residue: 0,
};

const SEGMENT_ALGO: crc::Algorithm<u32> = crc::Algorithm {
    width: 32,
    poly: 0x1EDC6F41,
    init: u32::MAX,
    refin: true,
    refout: true,
    xorout: 0,
    check: 0,
    residue: 0,
};

const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&ALGO);

const SEGMENT_CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&SEGMENT_ALGO);

#[test]
pub fn valid_frame() {
    let frame_data = &[
        01, 01, 36, 00, 00, 00, 08, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
        00, 00, 00, 00, 00, 63, 189, 107, 06, 01, 01, 01, 01, 28, 00, 00, 00, 02, 00, 00, 00, 00,
        00, 00, 00, 16, 00, 00, 00, 02, 00, 221, 90, 10, 00, 01, 05, 00, 00, 00, 00, 00, 00, 00,
        00, 00, 105, 92, 102, 236, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00,
    ];

    Frame::parse(&frame_data[..]).expect("Valid frame should be parseable");
}

#[test]
fn empty_segment_crc() {
    let empty_segment_crc = SEGMENT_CRC.checksum(&[]);

    assert_eq!(empty_segment_crc, u32::MAX);
}
