use crate::{DecodeError, frame::FrameFormat};

#[derive(Debug, Clone)]
pub struct Epilogue<'a> {
    pub late_flags: u8,
    pub crcs: &'a [u32],
}

impl<'a> Epilogue<'a> {
    pub const SERIALIZED_SIZE_V2_0: usize = 17;

    pub fn write(&self, mut output: impl std::io::Write) -> std::io::Result<usize> {
        output.write_all(&[self.late_flags])?;

        for crc in self.crcs.iter().copied() {
            output.write_all(&crc.to_le_bytes())?;
        }

        Ok(1 + 4 * 4)
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

    pub fn is_completed(&self, revision: FrameFormat) -> bool {
        match revision {
            FrameFormat::Rev0Crc => self.late_flags & 0x1 == 0x0,
            FrameFormat::Rev1Crc => self.late_flags & 0xF == 0xE,
            FrameFormat::Rev0Secure => todo!(),
            FrameFormat::Rev1Secure => todo!(),
        }
    }
}
