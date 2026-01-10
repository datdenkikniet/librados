use ceph_foundation::crypto::Key;

use crate::frame::Revision;

#[derive(Debug)]
pub struct EncryptError;

#[derive(Debug)]
pub struct DecryptError;

#[derive(Debug)]
pub struct FrameEncryption {
    inner: EncryptionInner,
}

impl Default for FrameEncryption {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameEncryption {
    pub const fn new() -> Self {
        Self {
            inner: EncryptionInner::None,
        }
    }

    pub fn session_key(&self) -> Option<&Key> {
        match &self.inner {
            EncryptionInner::None => None,
            EncryptionInner::CryptoKey { key, .. } => Some(key),
        }
    }

    pub fn is_secure(&self) -> bool {
        matches!(self.inner, EncryptionInner::CryptoKey { .. })
    }

    pub fn set_secret_data(
        &mut self,
        revision: Revision,
        key: Key,
        rx_nonce: [u8; 12],
        tx_nonce: [u8; 12],
    ) {
        self.inner = EncryptionInner::CryptoKey {
            key,
            rx_nonce: Nonce::new(rx_nonce),
            tx_nonce: Nonce::new(tx_nonce),
            revision,
        }
    }

    pub fn decrypt(&mut self, buffer: &mut [u8]) -> Result<usize, DecryptError> {
        match &mut self.inner {
            EncryptionInner::None => Ok(0),
            EncryptionInner::CryptoKey {
                key,
                rx_nonce,
                tx_nonce: _,
                revision,
            } => {
                if !buffer.is_empty() {
                    let in_len = buffer.len();
                    let nonce = rx_nonce.next(*revision).ok_or(DecryptError)?;
                    let out_len = key.decrypt_gcm(&nonce, buffer).ok_or(DecryptError)?.len();
                    in_len.checked_sub(out_len).ok_or(DecryptError)
                } else {
                    Ok(0)
                }
            }
        }
    }

    pub fn encrypt(&mut self, buffer: &mut [u8]) -> Result<[u8; 16], EncryptError> {
        match &mut self.inner {
            EncryptionInner::None => Ok([0u8; _]),
            EncryptionInner::CryptoKey {
                key,
                rx_nonce: _,
                tx_nonce,
                revision,
            } => {
                let nonce = tx_nonce.next(*revision).ok_or(EncryptError)?;
                Ok(key.encrypt_gcm(&nonce, buffer))
            }
        }
    }
}

enum EncryptionInner {
    None,
    CryptoKey {
        key: Key,
        rx_nonce: Nonce,
        tx_nonce: Nonce,
        revision: Revision,
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
                revision,
            } => f
                .debug_struct("CryptoKey")
                .field("key", key)
                .field("rx_nonce", &"<12 secret bytes>")
                .field("tx_nonce", &"<12 secret bytes>")
                .field("revision", revision)
                .finish(),
        }
    }
}

#[derive(Debug)]
struct Nonce {
    start: [u8; 12],
    current: [u8; 12],
    used_start: bool,
}

impl Nonce {
    fn new(nonce: [u8; 12]) -> Self {
        Self {
            start: nonce,
            current: nonce,
            used_start: false,
        }
    }

    fn next(&mut self, rev: Revision) -> Option<[u8; 12]> {
        let next = self.current;
        if next == self.start {
            if self.used_start {
                return None;
            }

            self.used_start = true;
        }

        match rev {
            Revision::Rev0 => todo!(),
            Revision::Rev1 => self.inc_rev1(),
        }

        Some(next)
    }

    fn inc_rev1(&mut self) {
        let current = self.current.last_chunk_mut().unwrap();
        let next = u64::from_le_bytes(*current).wrapping_add(1);
        *current = next.to_le_bytes();
    }
}
