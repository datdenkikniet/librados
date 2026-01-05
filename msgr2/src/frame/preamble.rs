use std::num::NonZeroU8;

use crate::{
    frame::{FrameFormat, epilogue::Epilogue},
    key::AES_GCM_SIG_SIZE,
};

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

const CRC: crc::Crc<u32> = crc::Crc::<u32>::new(&ALGO);

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Tag {
    Hello = 1,
    AuthRequest = 2,
    AuthBadMethod = 3,
    AuthReplyMore = 4,
    AuthRequestMore = 5,
    AuthDone = 6,
    AuthSignature = 7,
    ClientIdent = 8,
    ServerIdent = 9,
    IdentMissingFeatures = 10,
    SessionReconnect = 11,
    SessionReset = 12,
    SessionRetry = 13,
    SessionRetryGlobal = 14,
    SessionReconnectOk = 15,
    Wait = 16,
    Message = 17,
    Keepalive2 = 18,
    Keepalive2Ack = 19,
    Ack = 20,
    CompressionRequest = 21,
    CompressionDone = 22,
}

impl TryFrom<u8> for Tag {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let value = match value {
            1 => Self::Hello,
            2 => Self::AuthRequest,
            3 => Self::AuthBadMethod,
            4 => Self::AuthReplyMore,
            5 => Self::AuthRequestMore,
            6 => Self::AuthDone,
            7 => Self::AuthSignature,
            8 => Self::ClientIdent,
            9 => Self::ServerIdent,
            10 => Self::IdentMissingFeatures,
            11 => Self::SessionReconnect,
            12 => Self::SessionReset,
            13 => Self::SessionRetry,
            14 => Self::SessionRetryGlobal,
            15 => Self::SessionReconnectOk,
            16 => Self::Wait,
            17 => Self::Message,
            18 => Self::Keepalive2,
            19 => Self::Keepalive2Ack,
            20 => Self::Ack,
            21 => Self::CompressionRequest,
            22 => Self::CompressionDone,
            _ => return Err(()),
        };

        Ok(value)
    }
}

#[derive(Debug)]
pub struct Preamble {
    pub(crate) format: FrameFormat,
    pub(crate) tag: Tag,
    pub(crate) segment_count: NonZeroU8,
    pub(crate) segment_details: [SegmentDetail; 4],
    pub(crate) flags: u8,
    pub(crate) _reserved: u8,
    pub(crate) inline_data: Vec<u8>,
}

impl Preamble {
    pub const SERIALIZED_SIZE: usize = 32;
    pub const REV1_SECURE_INLINE_SIZE: usize = 48;

    pub fn len(format: &FrameFormat) -> usize {
        match format {
            crate::frame::FrameFormat::Rev0Crc => crate::frame::Preamble::SERIALIZED_SIZE,
            crate::frame::FrameFormat::Rev1Crc => crate::frame::Preamble::SERIALIZED_SIZE,
            crate::frame::FrameFormat::Rev0Secure => todo!(),
            crate::frame::FrameFormat::Rev1Secure => {
                Preamble::SERIALIZED_SIZE + Preamble::REV1_SECURE_INLINE_SIZE + AES_GCM_SIG_SIZE
            }
        }
    }

    pub fn data_and_epilogue_len(&self) -> usize {
        let segment_data: usize = self.segments().iter().map(|v| v.len()).sum();

        match self.format {
            FrameFormat::Rev0Crc => segment_data + Epilogue::SERIALIZED_SIZE_V2_0,
            FrameFormat::Rev1Crc => {
                let first_segment_crc = if self.segments()[0].len() > 0 { 4 } else { 0 };
                let epilogue = if self.segments().len() > 1 { 13 } else { 0 };

                segment_data + first_segment_crc + epilogue
            }
            FrameFormat::Rev0Secure => todo!(),
            FrameFormat::Rev1Secure => {
                let first_segment_data_len = {
                    let first_segment_len = self.segments()[0].len();
                    first_segment_len - (Self::REV1_SECURE_INLINE_SIZE.min(first_segment_len))
                };

                let other_segments_data_len: usize =
                    self.segments().iter().skip(1).map(|v| v.len()).sum();

                let data_len = first_segment_data_len + other_segments_data_len;

                if data_len > 0 {
                    data_len + AES_GCM_SIG_SIZE
                } else {
                    0
                }
            }
        }
    }

    pub fn write(&self, output: &mut Vec<u8>) {
        output.reserve(Self::SERIALIZED_SIZE);

        output.push(self.tag as _);
        output.push(self.segment_count.get());

        for (idx, detail) in self.segment_details.iter().enumerate() {
            if idx < self.segment_count.get() as usize {
                detail.write(output);
            } else {
                output.extend_from_slice(&[0u8; 6]);
            }
        }

        output.push(self.flags);

        // Reserved
        output.push(self._reserved);

        // Calculate crc
        let end = output.len();
        let start = output.len() - 28;
        let crc = CRC.checksum(&output[start..end]);
        output.extend_from_slice(&crc.to_le_bytes());
    }

    pub fn parse(
        input: &[u8; Self::SERIALIZED_SIZE],
        format: FrameFormat,
        mut inline_data: Vec<u8>,
    ) -> Result<Self, String> {
        let (tag_scount, buffer) = input.split_at(2);

        let Ok(tag) = Tag::try_from(tag_scount[0]) else {
            return Err(format!("Unknown tag value {}", tag_scount[0]));
        };

        let Some(segment_count) = NonZeroU8::new(tag_scount[1]) else {
            return Err("Segment count was zero".to_string());
        };

        if segment_count.get() > 4 {
            return Err(format!(
                "Segment count was greater than 4 ({segment_count})"
            ));
        }

        let (chunks, rest) = buffer.split_at(6 * 4);
        let (chunks, _) = chunks.as_chunks::<6>();

        let mut segment_details = [SegmentDetail::default(); 4];

        for i in 0..(segment_count.get() as usize) {
            segment_details[i] = SegmentDetail::parse(chunks[i]);
        }

        let flags = rest[0];
        let _reserved = rest[1];
        let crc = <[u8; 4]>::try_from(&rest[2..]).unwrap();
        let crc = u32::from_le_bytes(crc);

        let calculated_crc = CRC.checksum(&input[..28]);
        if calculated_crc != crc {
            return Err(format!(
                "Preamble CRC mismatch (received: 0x{crc:08X}, calculated: 0x{calculated_crc:08X}"
            ));
        }

        inline_data.truncate(segment_details[0].len());

        Ok(Self {
            format,
            tag,
            segment_count,
            segment_details,
            flags,
            _reserved,
            inline_data,
        })
    }

    pub(crate) fn segments(&self) -> &[SegmentDetail] {
        &self.segment_details[..self.segment_count.get() as usize]
    }

    pub fn inline_data(&mut self) -> Option<Vec<u8>> {
        match self.format {
            FrameFormat::Rev1Secure => Some(std::mem::take(&mut self.inline_data)),
            _ => None,
        }
    }

    pub fn has_non_inline_data(&self) -> bool {
        self.data_and_epilogue_len() != 0
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub(crate) struct SegmentDetail {
    pub length: u32,
    pub alignment: u16,
}

impl SegmentDetail {
    pub fn parse(buffer: [u8; 6]) -> Self {
        let length = <[u8; 4]>::try_from(&buffer[..4]).unwrap();
        let alignment = <[u8; 2]>::try_from(&buffer[4..]).unwrap();

        Self {
            length: u32::from_le_bytes(length),
            alignment: u16::from_le_bytes(alignment),
        }
    }

    pub fn write(&self, output: &mut Vec<u8>) {
        output.extend_from_slice(&self.length.to_le_bytes());
        output.extend_from_slice(&self.alignment.to_le_bytes());
    }

    pub fn len(&self) -> usize {
        self.length as _
    }

    #[expect(unused)]
    pub fn alignment(&self) -> usize {
        self.alignment as _
    }
}
