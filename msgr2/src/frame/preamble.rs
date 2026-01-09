use std::num::NonZeroU8;

use crate::frame::{Epilogue, FrameFormat, REV1_SECURE_INLINE_SIZE, REV1_SECURE_PAD_SIZE};

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
pub(crate) struct Preamble {
    pub format: FrameFormat,
    pub tag: Tag,
    pub segment_count: NonZeroU8,
    pub segment_details: [SegmentDetail; 4],
    pub flags: u8,
    pub _reserved: u8,
}

impl Preamble {
    pub const SERIALIZED_SIZE: usize = 32;

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

    pub fn parse(input: &[u8; Self::SERIALIZED_SIZE], format: FrameFormat) -> Result<Self, String> {
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

        Ok(Self {
            format,
            tag,
            segment_count,
            segment_details,
            flags,
            _reserved,
        })
    }

    pub fn write_epilogue(&self, crcs: &[u32], output: &mut Vec<u8>) {
        match self.format {
            FrameFormat::Rev0Crc => {
                let epilogue = Epilogue {
                    late_flags: 0,
                    crcs,
                };

                epilogue.write(output);
            }
            FrameFormat::Rev1Crc => {
                if self.need_epilogue_rev2_1() {
                    let epilogue = Epilogue {
                        late_flags: 0,
                        crcs: &crcs[1..],
                    };

                    epilogue.write(output);
                }
            }
            FrameFormat::Rev0Secure => todo!(),
            FrameFormat::Rev1Secure => {
                if self.need_epilogue_rev2_1() {
                    output.push(0xEu8);
                    output.extend_from_slice(&[0u8; 15]);
                }
            }
        };
    }

    pub fn data_and_epilogue_segments<'a>(&self) -> impl Iterator<Item = usize> + 'a + core::fmt::Debug {
        let expected_data = match self.format {
            FrameFormat::Rev0Crc => {
                let total_data_len: usize = self.segments().iter().map(|v| v.len()).sum();
                let data_and_epilogue = total_data_len + Epilogue::SERIALIZED_SIZE_V2_0_CRC;

                [Some(data_and_epilogue), None]
            }
            FrameFormat::Rev1Crc => {
                let total_data_len: usize = self.segments().iter().map(|v| v.len()).sum();

                let epilogue_len = if self.need_epilogue_rev2_1() {
                    4 + Epilogue::SERIALIZED_SIZE_V2_1_CRC
                } else {
                    4
                };

                let data_and_epilogue = total_data_len + epilogue_len;
                [Some(data_and_epilogue), None]
            }
            FrameFormat::Rev0Secure => todo!(),
            FrameFormat::Rev1Secure => {
                let mut segments = self.segments().iter();
                let seg1_len = segments.next().unwrap().len();

                // If the first segment is larger than the inline size, we should expect
                // a new block with the leftover data, with padding.
                let seg1_block = seg1_len
                    .checked_sub(REV1_SECURE_INLINE_SIZE)
                    .map(|seg1_left| seg1_left.next_multiple_of(REV1_SECURE_PAD_SIZE.get()));

                // If there are any follow-up segments, we should expect a single block, with:
                // 1. Data for each segment, with padding.
                // 2. An epilogue
                let other_segs_block = segments
                    .map(|v| v.len().next_multiple_of(REV1_SECURE_PAD_SIZE.get()))
                    .fold(None, |v, next| Some(v.unwrap_or(0) + next))
                    .map(|v| v + Epilogue::SERIALIZED_SIZE_V2_1_SECURE);

                [seg1_block, other_segs_block]
            }
        };

        expected_data.into_iter().flatten()
    }

    pub fn need_epilogue_rev2_1(&self) -> bool {
        self.segments().iter().skip(1).any(|v| v.len() > 0)
    }

    pub fn segments(&self) -> &[SegmentDetail] {
        &self.segment_details[..self.segment_count.get() as usize]
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
