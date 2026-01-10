use crate::frame::FrameFormat;

use ceph_foundation::DecodeError;

#[derive(Debug, Clone)]
pub struct Epilogue<'a> {
    pub late_flags: u8,
    pub crcs: &'a [u32],
}

impl<'a> Epilogue<'a> {
    pub const SERIALIZED_SIZE_V2_0_CRC: usize = 17;
    pub const SERIALIZED_SIZE_V2_1_CRC: usize = 13;
    pub const SERIALIZED_SIZE_V2_1_SECURE: usize = 16;

    pub fn write(&self, output: &mut Vec<u8>) {
        output.push(self.late_flags);

        for crc in self.crcs.iter().copied() {
            output.extend_from_slice(&crc.to_le_bytes());
        }
    }

    pub fn decode(data: &[u8], crcs: &'a mut [u32]) -> Result<Self, DecodeError> {
        let expected = 1 + (4 * crcs.len());
        if data.len() != expected {
            return Err(DecodeError::NotEnoughData {
                field: Some("epilogue"),
                have: data.len(),
                need: expected,
            });
        }

        let late_flags = data[0];

        for (idx, chunk) in data[1..].as_chunks().0.iter().enumerate() {
            let value = u32::from_le_bytes(*chunk);
            crcs[idx] = value;
        }

        Ok(Self { late_flags, crcs })
    }

    pub fn is_completed(&self, format: FrameFormat) -> bool {
        match format {
            FrameFormat::Rev0Crc => self.late_flags & 0x1 == 0x0,
            FrameFormat::Rev1Crc => self.late_flags & 0xF == 0xE,
            FrameFormat::Rev0Secure => todo!(),
            FrameFormat::Rev1Secure => self.late_flags & 0xF == 0xE,
        }
    }
}
