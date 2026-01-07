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

    pub fn set_secret_data(&mut self, key: CryptoKey, rx_nonce: [u8; 12], tx_nonce: [u8; 12]) {
        self.inner = EncryptionInner::CryptoKey {
            key,
            rx_nonce,
            tx_nonce,
        }
    }

    pub fn decrypt<'a>(&mut self, buffer: &'a mut [u8]) -> Option<usize> {
        match &self.inner {
            EncryptionInner::None => Some(0),
            EncryptionInner::CryptoKey {
                key,
                rx_nonce,
                tx_nonce: _,
            } => {
                if !buffer.is_empty() {
                    // TODO: increase rx nonce
                    let in_len = buffer.len();
                    let out_len = key.decrypt_gcm(&rx_nonce, buffer)?.len();
                    in_len.checked_sub(out_len)
                } else {
                    Some(0)
                }
            }
        }
    }
}

enum EncryptionInner {
    None,
    CryptoKey {
        key: CryptoKey,
        rx_nonce: [u8; 12],
        tx_nonce: [u8; 12],
    },
}

impl std::fmt::Debug for EncryptionInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::CryptoKey {
                key,
                rx_nonce: _,
                tx_nonce: _,
            } => f
                .debug_struct("CryptoKey")
                .field("key", key)
                .field("rx_nonce", &"<12 secret bytes>")
                .field("tx_nonce", &"<12 secret bytes>")
                .finish(),
        }
    }
}
