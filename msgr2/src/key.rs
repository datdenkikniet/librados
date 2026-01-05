use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};
use aes_gcm::{Aes128Gcm, KeyInit, aead::AeadMutInPlace};
use hmac::{Mac, digest::FixedOutput};

pub const CEPH_AES_IV: &[u8; 16] = b"cephsageyudagreg";
pub const AES_GCM_SIG_SIZE: usize = 16;

use crate::{Decode, DecodeError, Encode, Timestamp};

pub struct CryptoKey {
    ty: u16,
    created: Timestamp,
    secret: Vec<u8>,
}

impl core::fmt::Debug for CryptoKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CryptoKey")
            .field("ty", &self.ty)
            .field("created", &self.created)
            .field("secret", &format!("<{} secret bytes>", self.secret.len()))
            .finish()
    }
}

impl Encode for CryptoKey {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.ty.encode(buffer);
        self.created.encode(buffer);

        let len = u16::try_from(self.secret.len()).unwrap();
        len.encode(buffer);
        buffer.extend_from_slice(&self.secret);
    }
}

impl Decode<'_> for CryptoKey {
    fn decode(buffer: &mut &[u8]) -> Result<Self, DecodeError> {
        let ty = u16::decode(buffer)?;
        let created = Timestamp::decode(buffer)?;

        let len = u16::decode(buffer)?;

        let Some((secret, left)) = buffer.split_at_checked(len as usize) else {
            return Err(DecodeError::NotEnoughData {
                field: Some("secret"),
                have: buffer.len(),
                need: len as usize,
            });
        };

        *buffer = left;

        Ok(Self {
            ty,
            created,
            secret: secret.to_vec(),
        })
    }
}

impl CryptoKey {
    pub fn new(created: Timestamp, secret: [u8; 16]) -> Self {
        Self {
            ty: 1,
            created,
            secret: secret.to_vec(),
        }
    }

    pub fn hmac_sha256(&self, buf: &[u8]) -> [u8; 32] {
        let mut maybe_expected =
            <hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(&self.secret).unwrap();
        Mac::update(&mut maybe_expected, buf);
        maybe_expected.finalize_fixed().into()
    }

    pub fn encrypt(&self, data: &mut Vec<u8>) {
        // TODO: this is so bad...
        let secret: [u8; 16] = self.secret.as_slice().try_into().unwrap();
        let secret = secret.into();
        let data_len = data.len();

        // TODO: this is so bad...
        let iv = (*CEPH_AES_IV).into();
        let aes = cbc::Encryptor::<aes::Aes128>::new(&secret, &iv);
        data.resize(data_len + 16 * 2, 0);

        let res = aes.encrypt_padded_mut::<Pkcs7>(data, data_len).unwrap();
        let res_len = res.len();
        data.truncate(res_len);
    }

    pub fn decrypt<'a>(&self, data: &'a mut [u8]) -> Option<&'a [u8]> {
        // TODO: this is so bad...
        let secret: [u8; 16] = self.secret.as_slice().try_into().unwrap();
        let secret = secret.into();

        // TODO: this is so bad...
        let iv = (*CEPH_AES_IV).into();
        let aes = cbc::Decryptor::<aes::Aes128>::new(&secret, &iv);

        aes.decrypt_padded_mut::<Pkcs7>(data).ok()
    }

    pub fn decrypt_gcm<'a>(&self, nonce: &[u8; 12], data: &'a mut Vec<u8>) -> Option<&'a [u8]> {
        let mut gcm = Aes128Gcm::new_from_slice(&self.secret).unwrap();
        let nonce = (*nonce).into();
        gcm.decrypt_in_place(&nonce, &[], data).unwrap();
        Some(data)
    }
}

#[test]
fn decode_key() {
    let key_data = include_bytes!("./test.key");

    let key = CryptoKey::decode(&mut &key_data[..]);

    panic!("{key:?}");
}
