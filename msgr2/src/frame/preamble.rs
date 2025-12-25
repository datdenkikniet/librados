use std::num::NonZeroU8;

use crate::frame::epilogue::Epilogue;

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
    pub(crate) tag: Tag,
    pub(crate) segment_count: NonZeroU8,
    pub(crate) segment_details: [SegmentDetail; 4],
    pub(crate) flags: u8,
    pub(crate) _reserved: u8,
}

impl Preamble {
    pub const SERIALIZED_SIZE: usize = 32;

    pub fn data_and_epilogue_len(&self) -> usize {
        let segment_data: usize = self.segments().iter().map(|v| v.len()).sum();
        let epilogue_len = Epilogue::SERIALIZED_SIZE;

        segment_data + epilogue_len
    }

    pub fn write(&self, mut output: impl std::io::Write) -> std::io::Result<usize> {
        let mut buffer = [0u8; Self::SERIALIZED_SIZE];

        buffer[0] = self.tag as _;
        buffer[1] = self.segment_count.get();

        let mut used = 2;
        for (idx, detail) in self.segment_details.iter().enumerate() {
            let start = used;
            let end = start + 6;
            used += 6;

            let buffer = &mut buffer[start..end];

            if idx < self.segment_count.get() as usize {
                detail.write(buffer)?;
            } else {
                buffer.copy_from_slice(&[0u8; 6]);
            }
        }

        buffer[used] = self.flags;
        used += 1;

        // Reserved
        buffer[used] = self._reserved;
        used += 1;

        // Calculate crc
        assert_eq!(used, 28);
        let crc = CRC.checksum(&buffer[..used]);
        buffer[used..used + 4].copy_from_slice(&crc.to_le_bytes());

        output.write_all(&buffer)?;
        used += 4;

        Ok(used)
    }

    pub fn parse(input: &[u8]) -> Result<Self, String> {
        if input.len() != Self::SERIALIZED_SIZE {
            return Err(format!(
                "Expected 32 bytes of preamble data, got {}",
                input.len()
            ));
        }

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
            tag,
            segment_count,
            segment_details,
            flags,
            _reserved,
        })
    }

    pub(crate) fn segments(&self) -> &[SegmentDetail] {
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

    pub fn write(&self, mut output: impl std::io::Write) -> std::io::Result<usize> {
        output.write_all(&self.length.to_le_bytes())?;
        output.write_all(&self.alignment.to_le_bytes())?;
        Ok(6)
    }

    pub fn len(&self) -> usize {
        self.length as _
    }

    pub fn alignment(&self) -> usize {
        self.alignment as _
    }
}
