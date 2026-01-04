use crate::CryptoKey;

#[derive(Debug)]
pub struct Encryption {
    inner: EncryptionInner,
}

impl Encryption {
    pub fn new() -> Self {
        Self {
            inner: EncryptionInner::None,
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
