use crate::{Decode, DecodeError, Encode, Timestamp};

use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit, block_padding::Pkcs7};
use aes_gcm::{AeadCore, AeadInPlace, Aes128Gcm, KeyInit};
use hmac::{Mac, digest::FixedOutput};

pub const AUTH_MAGIC: u64 = 0xff009cad8826aa55;

pub const CEPH_AES_IV: &[u8; 16] = b"cephsageyudagreg";
pub const AES_GCM_SIG_SIZE: usize = 16;

pub fn encode_encrypt_enc_bl<T: Encode>(t: &T, key: &Key) -> Vec<u8> {
    let mut buffer = Vec::new();

    // Struct version
    buffer.push(1u8);
    AUTH_MAGIC.encode(&mut buffer);
    t.encode(&mut buffer);

    key.encrypt(&mut buffer);

    buffer
}

pub fn decode_decrypt_enc_bl<'a, T>(buf: &'a mut [u8], key: &Key) -> Result<T, DecodeError>
where
    T: Decode<'a> + 'a,
{
    let mut decrypted = key
        .decrypt(buf)
        .ok_or_else(|| DecodeError::Custom("Decryption failed".to_string()))?;

    let buf = &mut decrypted;

    let Some((v, left)) = buf.split_first() else {
        return Err(DecodeError::NotEnoughData {
            have: 0,
            need: 1,
            field: Some("encode_version"),
        });
    };

    if *v != 1 {
        return Err(DecodeError::UnexpectedVersion {
            ty: "encrypted",
            got: *v,
            expected: 1..=1,
        });
    }

    *buf = left;

    let magic = u64::decode(buf)?;

    if magic != AUTH_MAGIC {
        return Err(DecodeError::Custom(
            "Bad auth magic in decode_decrypt_enc_bl".to_string(),
        ));
    }

    T::decode(buf)
}

pub fn encode_encrypt<T: Encode>(t: &T, key: &Key) -> Vec<u8> {
    let encode_encrypt_bl = encode_encrypt_enc_bl(t, key);
    let mut encoded = Vec::new();
    encode_encrypt_bl.encode(&mut encoded);
    encoded
}

/// A cryptographic key.
///
/// This is the equivalent of the `Key` struct in the
/// ceph source code.
// TODO: zeroize...
pub struct Key {
    ty: u16,
    created: Timestamp,
    secret: Vec<u8>,
}

impl core::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Key")
            .field("ty", &self.ty)
            .field("created", &self.created)
            .field("secret", &format!("<{} secret bytes>", self.secret.len()))
            .finish()
    }
}

// TODO: this should not implement Encode directly...
impl Encode for Key {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.ty.encode(buffer);
        self.created.encode(buffer);

        let len = u16::try_from(self.secret.len()).unwrap();
        len.encode(buffer);
        buffer.extend_from_slice(&self.secret);
    }
}

impl Decode<'_> for Key {
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

impl Key {
    /// Create a new [`Key`].
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

    pub fn decrypt_gcm<'a>(&self, nonce: &[u8; 12], data: &'a mut [u8]) -> Option<&'a mut [u8]> {
        use aes::cipher::Unsigned;

        const TAG_SIZE: usize = <Aes128Gcm as AeadCore>::TagSize::USIZE;

        let len_min_tag = data.len().checked_sub(TAG_SIZE)?;

        let (data, tag) = data.split_at_mut(len_min_tag);

        let gcm = Aes128Gcm::new_from_slice(&self.secret).unwrap();
        // TODO: this is so bad...
        let nonce = (*nonce).into();
        let tag: [u8; TAG_SIZE] = tag.try_into().unwrap();

        gcm.decrypt_in_place_detached(&nonce, &[], data, &tag.into())
            .unwrap();

        Some(data)
    }

    pub fn encrypt_gcm(&self, nonce: &[u8; 12], data: &mut [u8]) -> [u8; 16] {
        let gcm = Aes128Gcm::new_from_slice(&self.secret).unwrap();
        let nonce = (*nonce).into();

        // This can only error if the frame is too big: fine by me for now.
        let tag = gcm.encrypt_in_place_detached(&nonce, &[], data).unwrap();

        tag.into()
    }
}

#[test]
fn decode_key() {
    let key_data = include_bytes!("./test.key");

    let key = Key::decode(&mut &key_data[..]).unwrap();

    assert_eq!(key.ty, 1);
    assert_eq!(
        key.created,
        Timestamp {
            tv_sec: 1763662875,
            tv_nsec: 702926448
        }
    );
    assert_eq!(
        key.secret,
        &[
            157, 25, 114, 34, 166, 24, 254, 3, 91, 218, 89, 106, 184, 116, 189, 55
        ]
    );
}
