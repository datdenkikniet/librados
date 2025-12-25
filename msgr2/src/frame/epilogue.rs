#[derive(Debug, Clone)]
pub struct Epilogue {
    pub late_flags: u8,
    pub crcs: [u32; 4],
}

impl Epilogue {
    pub const SERIALIZED_SIZE: usize = 17;

    pub fn write(&self, mut output: impl std::io::Write) -> std::io::Result<usize> {
        output.write_all(&[self.late_flags])?;

        for crc in self.crcs.iter().copied() {
            output.write_all(&crc.to_le_bytes())?;
        }

        Ok(1 + 4 * 4)
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
