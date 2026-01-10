mod encryption;
mod epilogue;
mod preamble;
mod wire;

use std::num::{NonZeroU8, NonZeroUsize};

pub use encryption::FrameEncryption;
pub use epilogue::Epilogue;
pub use preamble::{Preamble, Tag};

pub use encryption::{DecryptError, EncryptError};
pub use wire::{Completed, ReadPreamble, RxError, RxFrame, TxError, TxFrame, Unstarted};

use crate::frame::preamble::SegmentDetail;

use ceph_foundation::DecodeError;

pub const REV1_SECURE_INLINE_SIZE: usize = 48;
pub const REV1_SECURE_PAD_SIZE: NonZeroUsize = NonZeroUsize::new(16).unwrap();

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

    pub const fn segment_pad_size(&self) -> NonZeroUsize {
        match self {
            FrameFormat::Rev0Crc | FrameFormat::Rev1Crc => NonZeroUsize::new(1).unwrap(),
            FrameFormat::Rev0Secure => todo!(),
            FrameFormat::Rev1Secure => REV1_SECURE_PAD_SIZE,
        }
    }

    pub fn revision(&self) -> Revision {
        match self {
            FrameFormat::Rev0Crc | Self::Rev0Secure => Revision::Rev0,
            FrameFormat::Rev1Crc | Self::Rev1Secure => Revision::Rev1,
        }
    }
}

/// The `msgr2` revision a connection has.
#[derive(Debug, Clone, Copy)]
pub enum Revision {
    /// Revision 2.0
    Rev0,
    /// Revision 2.1
    Rev1,
}

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

const EMPTY: &[u8] = &[];
const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&ALGO);

#[derive(Debug, Clone)]
pub struct Frame<'a> {
    tag: Tag,
    valid_segments: NonZeroU8,
    segments: [&'a [u8]; 4],
}

impl<'a> Frame<'a> {
    pub fn send<'frame>(
        &self,
        format: FrameFormat,
        encryption: &'frame mut FrameEncryption,
        buffer: &'frame mut Vec<u8>,
    ) -> TxFrame<'frame> {
        self.write(format, buffer);

        TxFrame {
            preamble: self.preamble(format),
            enc: encryption,
            frame_data: buffer,
        }
    }

    pub fn new(tag: Tag, segments: &[&'a [u8]]) -> Result<Self, String> {
        if segments.is_empty() || segments.len() > 4 {
            return Err("Invalid amount of segments".to_string());
        }

        if segments.last().map(|v| v.is_empty()).unwrap_or(false) {
            return Err("Last segment in list was empty".to_string());
        }

        let valid_segments = NonZeroU8::new(segments.len() as _).unwrap();

        let mut segments_out = [EMPTY; 4];
        segments_out[..segments.len()].copy_from_slice(segments);

        Ok(Self {
            tag,
            valid_segments,
            segments: segments_out,
        })
    }

    pub(crate) fn preamble(&self, format: FrameFormat) -> Preamble {
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

    /// Write this `Frame` to `output`, with preamble, frame data, optional
    /// epilogue and optional padding.
    ///
    /// You should not send `output` anywhere directly: in order for the frame
    /// to be encrypted according to the format and current [`FrameEncryption`],
    /// use [`Frame::send`] instead.
    pub fn write(&self, format: FrameFormat, output: &mut Vec<u8>) {
        let preamble = self.preamble(format);
        preamble.write(output);

        let mut crcs = [0u32; 4];

        for (idx, segment) in self.segments().enumerate() {
            let crc = CRC.checksum(segment);
            crcs[idx] = crc;
            output.extend_from_slice(segment);

            // Apply padding
            let pad_size = format.segment_pad_size().get();
            let full_pad_len = segment.len().next_multiple_of(pad_size);
            let required_padding = full_pad_len - segment.len();
            output.extend(core::iter::repeat_n(0, required_padding));

            if format == FrameFormat::Rev1Crc && idx == 0 && !segment.is_empty() {
                output.extend_from_slice(&crc.to_le_bytes());
            }
        }

        preamble.write_epilogue(crcs.as_slice(), output);
    }

    pub fn decode(preamble: &Preamble, data: &'a [u8]) -> Result<Self, DecodeError> {
        let mut trailer = data;

        let mut segments = [EMPTY; 4];
        let mut crc_segment1 = None;

        fn split_segment(buf: &[u8], len: usize) -> Result<(&[u8], &[u8]), DecodeError> {
            let err = || DecodeError::NotEnoughData {
                field: Some("segment"),
                have: buf.len(),
                need: len,
            };

            buf.split_at_checked(len).ok_or_else(err)
        }

        segments[0] = {
            let segment0 = preamble.segments()[0];
            let padded_len = segment0
                .len()
                .next_multiple_of(preamble.format.segment_pad_size().get());

            let (mut segment_data, left) = split_segment(trailer, padded_len)?;
            // Shorten to drop potential padding
            segment_data = &segment_data[..segment0.len()];
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

            segment_data
        };

        for (idx, segment) in preamble.segments().iter().enumerate().skip(1) {
            let padded_len = segment
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
                            "Trailing epilogue bytes were not zeroed".to_string(),
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
                    let calculated_crc = CRC.checksum(segment);
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
                        "Found non-zero CRC (0x{:08X}) for a trailing segment (#{})",
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
