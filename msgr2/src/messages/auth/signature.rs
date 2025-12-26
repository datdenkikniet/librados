use crate::EncodeExt;

#[derive(Debug, Clone)]
pub struct AuthSignature {
    pub sha256: [u8; 32],
}

impl EncodeExt for AuthSignature {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.sha256.encode(buffer);
    }
}

impl AuthSignature {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() != 32 {
            return Err(format!(
                "Expected {} bytes of signature data, got only {}",
                32,
                data.len()
            ));
        }

        let sha256 = data.try_into().unwrap();

        Ok(Self { sha256 })
    }
}
