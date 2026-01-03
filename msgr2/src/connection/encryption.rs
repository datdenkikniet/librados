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

    pub fn set_secret_key(&mut self, key: CryptoKey) {
        self.inner = EncryptionInner::CryptoKey(key)
    }
}

#[derive(Debug)]
enum EncryptionInner {
    None,
    CryptoKey(CryptoKey),
}
