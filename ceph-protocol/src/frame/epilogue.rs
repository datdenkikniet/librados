pub struct Epilogue {
    pub late_flags: u8,
    pub crcs: [u32; 4],
}

impl Epilogue {
    pub const LEN: usize = 17;

    pub fn write(&self, buffer: &mut [u8]) -> Result<usize, String> {
        if buffer.len() < Self::LEN {
            return Err(format!(
                "Expected buffer with at least {} bytes, only got {}",
                Self::LEN,
                buffer.len()
            ));
        }

        buffer[0] = self.late_flags;

        for (idx, crc) in self.crcs.iter().copied().enumerate() {
            let start = 1 + (idx * 4);
            let end = start + 4;
            buffer[start..end].copy_from_slice(&crc.to_le_bytes());
        }

        Ok(17)
    }

    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() != 17 {
            return Err(format!(
                "Expected 17 bytes of epilogue data, got {}",
                data.len()
            ));
        }

        let late_flags = data[0];
        let mut crcs = [0u32; 4];

        for (idx, chunk) in data[1..].chunks_exact(4).enumerate() {
            let value = u32::from_le_bytes(chunk.try_into().unwrap());
            crcs[idx] = value;
        }

        Ok(Self { late_flags, crcs })
    }
}
