use std::num::NonZeroU8;

use crate::{
    DecodeError,
    frame::{
        FrameFormat,
        epilogue::Epilogue,
        preamble::{Preamble, SegmentDetail, Tag},
    },
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

const EMPTY: &'static [u8] = &[];
const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&ALGO);

#[derive(Debug, Clone)]
pub(crate) struct Frame<'a> {
    tag: Tag,
    valid_segments: NonZeroU8,
    segments: [&'a [u8]; 4],
}

impl<'a> Frame<'a> {
    pub(crate) fn new(tag: Tag, segments: &[&'a [u8]]) -> Option<Self> {
        if segments.is_empty() || segments.len() > 4 {
            return None;
        }

        let valid_segments = NonZeroU8::new(segments.len() as _).unwrap();

        let mut segments_out = [EMPTY; 4];
        segments_out[..segments.len()].copy_from_slice(segments);

        Some(Self {
            tag,
            valid_segments,
            segments: segments_out,
        })
    }

    pub fn preamble(&self, format: FrameFormat) -> Preamble {
        let mut segment_details = [SegmentDetail::default(); 4];
        for (idx, segment) in self.segments().enumerate() {
            segment_details[idx] = SegmentDetail {
                length: segment.len() as _,
                alignment: 1,
            };
        }

        Preamble {
            format,
            flags: 0,
            tag: self.tag,
            segment_count: self.valid_segments,
            segment_details,
            _reserved: 0,
        }
    }

    pub fn write(&self, format: FrameFormat, output: &mut Vec<u8>) {
        let preamble = self.preamble(format);
        preamble.write(output);

        let mut crcs = [0u32; 4];

        for (idx, segment) in self.segments().enumerate() {
            let crc = CRC.checksum(segment);
            crcs[idx] = crc;
            output.extend_from_slice(segment);

            if format == FrameFormat::Rev1Crc && idx == 0 && segment.len() > 0 {
                output.extend_from_slice(&crc.to_le_bytes());
            }
        }

        preamble.write_epilogue(crcs.as_slice(), output);
    }

    pub fn decode(preamble: &Preamble, data: &'a [u8]) -> Result<Self, DecodeError> {
        let mut trailer = data;

        let mut segments = [EMPTY; 4];
        let mut crc_segment1 = None;

        fn split_segment<'a>(
            buf: &'a [u8],
            len: usize,
        ) -> Result<(&'a [u8], &'a [u8]), DecodeError> {
            let err = || DecodeError::NotEnoughData {
                field: Some("segment"),
                have: buf.len(),
                need: len,
            };

            buf.split_at_checked(len).ok_or_else(err)
        }

        let segment0 = preamble.segments()[0];
        segments[0] = {
            let (segment, left) = split_segment(trailer, segment0.len())?;
            trailer = left;

            if preamble.format == FrameFormat::Rev1Crc {
                let err = || DecodeError::NotEnoughData {
                    field: Some("crc1"),
                    have: trailer.len(),
                    need: 4,
                };

                let (crc, left) = trailer.split_first_chunk::<4>().ok_or_else(err)?;

                crc_segment1 = Some(u32::from_le_bytes(*crc));
                trailer = left;
            }

            segment
        };

        for (idx, segment) in preamble.segments().iter().enumerate().skip(1) {
            let padded_len = segment0
                .len()
                .next_multiple_of(preamble.format.segment_pad_size().get());

            let (segment_data, left) = split_segment(trailer, padded_len)?;

            // Shorten to drop potential padding
            segments[idx] = &segment_data[..segment.len()];
            trailer = left;
        }

        Self::handle_epilogue(preamble, crc_segment1, &segments, trailer)?;

        Ok(Self {
            tag: preamble.tag,
            valid_segments: preamble.segment_count,
            segments,
        })
    }

    fn handle_epilogue(
        preamble: &Preamble,
        crc_segment1: Option<u32>,
        segments: &[&[u8]; 4],
        trailer: &[u8],
    ) -> Result<(), DecodeError> {
        let mut crcs = [0; 4];

        let completed = match preamble.format {
            FrameFormat::Rev0Crc => {
                let epilogue = Epilogue::decode(trailer, &mut crcs)?;
                epilogue.is_completed(preamble.format)
            }
            FrameFormat::Rev1Crc => {
                crcs[0] = crc_segment1.unwrap_or(0xFFFF_FFFF);

                if preamble.need_epilogue_rev2_1() {
                    let epilogue = Epilogue::decode(trailer, &mut crcs[1..])?;
                    epilogue.is_completed(preamble.format)
                } else if !trailer.is_empty() {
                    return Err(DecodeError::Custom(format!(
                        "Epilogue should have been empty, but had {} trailing bytes",
                        trailer.len()
                    )));
                } else {
                    true
                }
            }
            FrameFormat::Rev0Secure => todo!(),
            FrameFormat::Rev1Secure => {
                if preamble.need_epilogue_rev2_1() {
                    if trailer.len() != 16 {
                        return Err(DecodeError::Custom(format!(
                            "Expected 16 bytes of epilogue data, got {}",
                            trailer.len()
                        )));
                    }

                    if !trailer[1..].iter().all(|v| *v == 0) {
                        return Err(DecodeError::Custom(
                            "Trailing epilogue bytes were not zeroed.".to_string(),
                        ));
                    }

                    let epilogue = Epilogue::decode(&trailer[..1], &mut [])?;
                    epilogue.is_completed(preamble.format)
                } else if !trailer.is_empty() {
                    return Err(DecodeError::Custom(format!(
                        "Epilogue should have been empty, but had {} trailing bytes",
                        trailer.len()
                    )));
                } else {
                    true
                }
            }
        };

        if !completed {
            return Err(DecodeError::Custom(
                "Epilogue status did not indicate correct completion".to_string(),
            ));
        }

        if preamble.format.has_crc() {
            for (idx, crc) in crcs.iter().copied().enumerate() {
                if idx < preamble.segment_count.get() as usize {
                    let segment = &segments[idx];
                    let calculated_crc = CRC.checksum(&segment);
                    if crc != calculated_crc {
                        return Err(DecodeError::Custom(format!(
                            "Found incorrect CRC 0x{:08X} (expected 0x{:08X}) for segment (#{})",
                            crc,
                            calculated_crc,
                            idx + 1
                        )));
                    }
                } else if crc != 0 {
                    return Err(DecodeError::Custom(format!(
                        "Found non-zero CRC (0x{:08X}) for a trailing segment (#{}).",
                        crc,
                        idx + 1
                    )));
                }
            }
        }

        Ok(())
    }

    pub const fn tag(&self) -> Tag {
        self.tag
    }

    pub fn segments(&self) -> impl Iterator<Item = &[u8]> {
        self.segments[..self.valid_segments.get() as usize]
            .iter()
            .map(|v| v.as_ref())
    }
}
