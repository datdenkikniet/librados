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

    pub fn parse(data: &[u8], crcs: &'a mut [u32]) -> Result<Self, String> {
        let expected = 1 + (4 * crcs.len());
        if data.len() != expected {
            return Err(format!(
                "Expected {expected} bytes of epilogue data, got {}",
                data.len()
            ));
        }

        let late_flags = data[0];

        for (idx, chunk) in data[1..].as_chunks().0.iter().enumerate() {
            let value = u32::from_le_bytes(*chunk);
            crcs[idx] = value;
        }

        Ok(Self { late_flags, crcs })
    }
}
