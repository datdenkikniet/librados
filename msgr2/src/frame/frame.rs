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

const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&ALGO);

#[derive(Debug, Clone)]
enum VecOrSlice<'a> {
    Vec(Vec<u8>),
    Slice(&'a [u8]),
}

impl core::ops::Deref for VecOrSlice<'_> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            VecOrSlice::Vec(items) => items.as_slice(),
            VecOrSlice::Slice(items) => items,
        }
    }
}

#[derive(Debug)]
pub struct Frame<'a> {
    pub(crate) data: &'a mut Vec<u8>,
}

impl Frame<'_> {
    pub fn write(self, mut output: impl std::io::Write) -> std::io::Result<usize> {
        output.write_all(&self.data)?;
        Ok(self.data.len())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ParsedFrame<'a> {
    format: FrameFormat,
    tag: Tag,
    valid_segments: NonZeroU8,
    segments: [VecOrSlice<'a>; 4],
}

impl<'a> ParsedFrame<'a> {
    pub(crate) fn new(tag: Tag, segments: &[&'a [u8]], format: FrameFormat) -> Option<Self> {
        if segments.len() == 0 || segments.len() > 4 {
            return None;
        }

        let valid_segments = NonZeroU8::new(segments.len() as _).unwrap();

        let mut segments_out = [
            VecOrSlice::Slice(&[]),
            VecOrSlice::Slice(&[]),
            VecOrSlice::Slice(&[]),
            VecOrSlice::Slice(&[]),
        ];

        for (input, output) in segments.iter().zip(segments_out.iter_mut()) {
            *output = VecOrSlice::Slice(input);
        }

        Some(Self {
            format,
            tag,
            valid_segments,
            segments: segments_out,
        })
    }

    pub fn write(&self, output: &mut Vec<u8>) {
        let mut segment_details = [SegmentDetail::default(); 4];
        for (idx, segment) in self.segments().enumerate() {
            segment_details[idx] = SegmentDetail {
                length: segment.len() as _,
                alignment: 1,
            };
        }

        let preamble = Preamble {
            format: self.format,
            flags: 0,
            tag: self.tag,
            segment_count: self.valid_segments,
            segment_details,
            _reserved: 0,
            inline_data: Vec::new(),
        };

        preamble.write(output);
        let mut crcs = [0u32; 4];

        for (idx, segment) in self.segments().enumerate() {
            let crc = CRC.checksum(segment);
            crcs[idx] = crc;
            output.extend_from_slice(segment);

            if self.format == FrameFormat::Rev1Crc && idx == 0 && segment.len() > 0 {
                output.extend_from_slice(&crc.to_le_bytes());
            }
        }

        match self.format {
            FrameFormat::Rev0Crc => {
                let epilogue = Epilogue {
                    late_flags: 0,
                    crcs: &crcs,
                };

                epilogue.write(output);
            }
            FrameFormat::Rev1Crc => {
                let need_epilogue = self.segments().skip(1).any(|v| v.len() > 0);

                if need_epilogue {
                    let epilogue = Epilogue {
                        late_flags: 0,
                        crcs: &crcs[1..],
                    };

                    epilogue.write(output);
                }
            }
            FrameFormat::Rev0Secure => todo!(),
            FrameFormat::Rev1Secure => {
                // TODO: this is quite incorrect xd
            }
        };
    }

    pub fn decode(preamble: &'a mut Preamble, data: &'a [u8]) -> Result<Self, DecodeError> {
        use VecOrSlice::Slice;

        let mut trailer = data;

        let mut segments = [Slice(&[]), Slice(&[]), Slice(&[]), Slice(&[])];
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
            let inline_data = preamble.inline_data();
            let len = if let Some(len) = inline_data.as_ref().map(|v| v.len()) {
                segment0.len().saturating_sub(len)
            } else {
                segment0.len()
            };

            let (segment, left) = split_segment(trailer, len)?;
            trailer = left;

            let out = if let Some(mut inline_data) = inline_data {
                inline_data.extend_from_slice(segment);
                VecOrSlice::Vec(inline_data)
            } else {
                VecOrSlice::Slice(segment)
            };

            if preamble.format == FrameFormat::Rev1Crc {
                let err = || DecodeError::NotEnoughData {
                    field: Some("crc1"),
                    have: trailer.len(),
                    need: len,
                };

                let (crc, left) = trailer.split_first_chunk::<4>().ok_or_else(err)?;

                crc_segment1 = Some(u32::from_le_bytes(*crc));
                trailer = left;
            }

            out
        };

        for (idx, segment) in preamble.segments().iter().enumerate().skip(1) {
            let (segment, left) = split_segment(trailer, segment.len())?;

            trailer = left;
            segments[idx] = VecOrSlice::Slice(segment);
        }

        Self::handle_epilogue(preamble, crc_segment1, &segments, trailer)?;

        Ok(Self {
            format: preamble.format,
            tag: preamble.tag,
            valid_segments: preamble.segment_count,
            segments,
        })
    }

    fn handle_epilogue(
        preamble: &Preamble,
        crc_segment1: Option<u32>,
        segments: &[VecOrSlice; 4],
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

                if preamble.segments().iter().skip(1).any(|v| v.len() > 0) {
                    let epilogue = Epilogue::decode(trailer, &mut crcs[1..])?;
                    epilogue.is_completed(preamble.format)
                } else {
                    true
                }
            }
            FrameFormat::Rev0Secure => todo!(),
            FrameFormat::Rev1Secure => {
                if !trailer.is_empty() {
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
