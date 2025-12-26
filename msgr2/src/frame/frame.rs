use std::num::NonZeroU8;

use crate::frame::{
    Msgr2Revision,
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

#[derive(Debug, Clone)]
pub struct Frame<'a> {
    revision: Msgr2Revision,
    tag: Tag,
    valid_segments: NonZeroU8,
    segments: [&'a [u8]; 4],
}

impl<'a> Frame<'a> {
    pub fn new(tag: Tag, segments: &[&'a [u8]], revision: Msgr2Revision) -> Option<Self> {
        if segments.len() == 0 || segments.len() > 4 {
            return None;
        }

        let valid_segments = NonZeroU8::new(segments.len() as _).unwrap();

        let mut segments_out = [EMPTY; 4];
        segments_out[..segments.len()].copy_from_slice(segments);

        Some(Self {
            revision,
            tag,
            valid_segments,
            segments: segments_out,
        })
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut output = Vec::new();
        self.write(&mut output).unwrap();
        output
    }

    pub fn write(&self, mut output: impl std::io::Write) -> std::io::Result<usize> {
        let segments = self.segments();

        let mut segment_details = [SegmentDetail::default(); 4];
        for (idx, segment) in segments.iter().enumerate() {
            segment_details[idx] = SegmentDetail {
                length: segment.len() as _,
                alignment: 1,
            };
        }

        let preamble = Preamble {
            revision: self.revision,
            flags: 0,
            tag: self.tag,
            segment_count: self.valid_segments,
            segment_details,
            _reserved: 0,
        };

        let mut used = preamble.write(&mut output)?;
        let mut crcs = [0u32; 4];

        for (idx, segment) in segments.iter().enumerate() {
            let crc = CRC.checksum(segment);
            crcs[idx] = crc;
            output.write_all(segment)?;
            used += segment.len();

            if self.revision == Msgr2Revision::V2_1 && idx == 0 && segment.len() > 0 {
                output.write_all(&crc.to_le_bytes())?;
                used += 4;
            }
        }

        used += match self.revision {
            Msgr2Revision::V2_0 => {
                let epilogue = Epilogue {
                    late_flags: 0,
                    crcs: &crcs,
                };

                epilogue.write(&mut output)?
            }
            Msgr2Revision::V2_1 => {
                let need_epilogue = segments.iter().skip(1).any(|v| v.len() > 0);

                if need_epilogue {
                    let epilogue = Epilogue {
                        late_flags: 0,
                        crcs: &crcs[1..],
                    };

                    epilogue.write(&mut output)?
                } else {
                    0
                }
            }
        };

        Ok(used)
    }

    pub fn parse(preamble: &Preamble, data: &'a [u8]) -> Result<Self, String> {
        let mut trailer = data;

        let mut segments = [EMPTY; 4];
        let mut crc_segment1 = 0xFFFF_FFFF;

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

            if idx == 0 && preamble.revision == Msgr2Revision::V2_1 {
                let (crc, left) = trailer.split_first_chunk::<4>().ok_or_else(|| {
                    format!(
                        "Expected 4 bytes of CRC data, but only had {} left",
                        trailer.len()
                    )
                })?;

                crc_segment1 = u32::from_le_bytes(*crc);
                trailer = left;
            }
        }

        let mut crcs = [0; 4];

        match preamble.revision {
            Msgr2Revision::V2_0 => {
                Epilogue::parse(trailer, &mut crcs)?;
            }
            Msgr2Revision::V2_1 => {
                crcs[0] = crc_segment1;

                if preamble.segments().iter().skip(1).any(|v| v.len() > 0) {
                    Epilogue::parse(trailer, &mut crcs[1..])?;
                }
            }
        };

        for (idx, crc) in crcs.iter().copied().enumerate() {
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
            } else if crc != 0 {
                return Err(format!(
                    "Found non-zero CRC (0x{:08X}) for a trailing segment (#{}).",
                    crc,
                    idx + 1
                ));
            }
        }

        Ok(Self {
            revision: preamble.revision,
            tag: preamble.tag,
            valid_segments: preamble.segment_count,
            segments,
        })
    }

    pub const fn tag(&self) -> Tag {
        self.tag
    }

    pub fn segments(&self) -> &[&[u8]] {
        &self.segments[..self.valid_segments.get() as usize]
    }
}
