#[derive(Debug, Clone)]
pub struct AuthSignature {
    pub sha256: [u8; 32],
}

write_decode_encode!(AuthSignature = sha256);
