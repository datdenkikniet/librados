use crate::CryptoKey;

#[derive(Debug)]
pub struct FrameEncryption {
    inner: EncryptionInner,
}

impl FrameEncryption {
    pub fn new() -> Self {
        Self {
            inner: EncryptionInner::None,
        }
    }

    pub fn session_key(&self) -> Option<&CryptoKey> {
        match &self.inner {
            EncryptionInner::None => None,
            EncryptionInner::CryptoKey { key, .. } => Some(key),
        }
    }

    pub fn is_secure(&self) -> bool {
        matches!(self.inner, EncryptionInner::CryptoKey { .. })
    }

    pub fn set_secret_data(&mut self, key: CryptoKey, nonce: [u8; 12]) {
        self.inner = EncryptionInner::CryptoKey { key, nonce }
    }

    pub fn decrypt(&self, buffer: &mut Vec<u8>) {
        match &self.inner {
            EncryptionInner::None => {}
            EncryptionInner::CryptoKey { key, nonce } => {
                key.decrypt_gcm(&nonce, buffer).unwrap();
            }
        }
    }
}

#[derive(Debug)]
enum EncryptionInner {
    None,
    CryptoKey { key: CryptoKey, nonce: [u8; 12] },
}
