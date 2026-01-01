use aes::cipher::{
    BlockEncryptMut, KeyIvInit,
    block_padding::{Padding, Pkcs7},
};

pub const CEPH_AES_IV: &[u8; 16] = b"cephsageyudagreg";

use crate::{Encode, Timestamp};

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

impl CryptoKey {
    pub fn encode_hazmat(&self, buffer: &mut Vec<u8>) {
        self.ty.encode(buffer);
        self.created.encode(buffer);

        let len = u16::try_from(self.secret.len()).unwrap();
        len.encode(buffer);
        buffer.extend_from_slice(&self.secret);
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

    pub fn decode(data: &[u8]) -> Result<Self, String> {
        let Some((ty, data)) = data.split_first_chunk::<2>() else {
            return Err(format!("Expected 2 bytes for key type."));
        };

        let ty = u16::from_le_bytes(*ty);
        let (created, used) = Timestamp::parse(data).unwrap();
        let data = &data[used..];

        let Some((len, data)) = data.split_first_chunk::<2>() else {
            return Err(format!("Expected 2 bytes for key len."));
        };
        let len = u16::from_le_bytes(*len);

        if data.len() != len as usize {
            return Err(format!(
                "Expected {} bytes of key data, only got {}",
                len,
                data.len()
            ));
        }

        Ok(Self {
            ty,
            created,
            secret: data.to_vec(),
        })
    }
}

#[test]
fn decode_key() {
    let key_data = include_bytes!("./test.key");

    let key = CryptoKey::decode(key_data);

    panic!("{key:?}");
}
