use std::num::NonZeroU8;

use crate::frame::{
    epilogue::Epilogue,
    preamble::{Preamble, SegmentDetail, Tag},
};

const ALGO: crc::Algorithm<u32> = crc::Algorithm {
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

const EMPTY: &'static [u8] = &[];

#[derive(Debug)]
pub struct Frame<'a> {
    tag: Tag,
    valid_segments: NonZeroU8,
    segments: [&'a [u8]; 4],
}

impl<'a> Frame<'a> {
    pub fn new(tag: Tag, segments: &[&'a [u8]]) -> Option<Self> {
        if segments.len() == 0 || segments.len() > 4 {
            return None;
        }

        assert!(segments.len() <= 4);
        let valid_segments = NonZeroU8::new(segments.len() as _).unwrap();

        let mut segments_out = [EMPTY; 4];
        segments_out[..segments.len()].copy_from_slice(segments);

        Some(Self {
            tag,
            valid_segments,
            segments: segments_out,
        })
    }

    pub fn write(&self, buffer: &mut [u8]) -> Result<usize, String> {
        let segments = &self.segments[..self.valid_segments.get() as usize];

        let mut segment_details = [SegmentDetail::default(); 4];
        for (idx, segment) in segments.iter().enumerate() {
            segment_details[idx] = SegmentDetail {
                length: segment.len() as _,
                alignment: 1,
            };
        }

        let preamble = Preamble {
            flags: 0,
            tag: self.tag,
            segment_count: self.valid_segments,
            segment_details,
            _reserved: 0,
        };

        let mut used = preamble.write(buffer)?;
        let (_, mut buffer) = buffer.split_at_mut(used);
        let mut crcs = [0u32; 4];

        for (idx, segment) in segments.iter().enumerate() {
            if buffer.len() < segment.len() {
                return Err(format!(
                    "Expected buffer of at least {} bytes to write segment, only got {}",
                    segment.len(),
                    buffer.len()
                ));
            }

            crcs[idx] = CRC.checksum(segment);
            buffer[..segment.len()].copy_from_slice(segment);
            buffer = &mut buffer[segment.len()..];
            used += segment.len();
        }

        let epilogue = Epilogue {
            late_flags: 0,
            crcs,
        };

        used += epilogue.write(buffer)?;

        Ok(used)
    }

    pub fn parse(data: &'a [u8]) -> Result<Self, String> {
        if data.len() < Preamble::LEN + Epilogue::LEN {
            return Err(format!(
                "Expected at least {} preamble and epilogue bytes, got {}",
                Preamble::LEN + Epilogue::LEN,
                data.len()
            ));
        }

        let (preamble, mut trailer) = data.split_at(32);
        let preamble = Preamble::parse(&preamble)?;

        let mut segments = [EMPTY; 4];

        for (idx, segment) in preamble.segments().iter().enumerate() {
            let len = segment.len();

            let (segment, left) = trailer.split_at_checked(len).ok_or_else(|| {
                format!(
                    "Expected {} bytes of segment data, but only had {} left",
                    len,
                    trailer.len()
                )
            })?;
            trailer = left;
            segments[idx] = segment;
        }

        let epilogue = Epilogue::parse(trailer)?;

        for (idx, crc) in epilogue.crcs.iter().copied().enumerate() {
            if idx < preamble.segment_count.get() as usize {
                let segment = segments[idx];
                let calculated_crc = CRC.checksum(segment);
                if crc != calculated_crc {
                    return Err(format!(
                        "Found incorrect CRC 0x{:08X} (expected 0x{:08X}) for segment (#{})",
                        crc,
                        calculated_crc,
                        idx + 1
                    ));
                }
            } else {
                if crc != 0 {
                    return Err(format!(
                        "Found non-zero CRC (0x{:08X}) for a trailing segment (#{}).",
                        crc,
                        idx + 1
                    ));
                }
            }
        }

        Ok(Self {
            tag: preamble.tag,
            valid_segments: preamble.segment_count,
            segments,
        })
    }

    pub fn segments(&self) -> &[&[u8]] {
        &self.segments[..self.valid_segments.get() as usize]
    }
}
