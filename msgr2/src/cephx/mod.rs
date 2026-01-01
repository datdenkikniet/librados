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

    pub fn decode(data: &[u8]) -> Self {
        let ty = u16::from_le_bytes(data[..2].try_into().unwrap());
        let (created, used) = Timestamp::parse(&data[2..]).unwrap();
        let data = &data[2 + used..];

        let len = u16::from_le_bytes(data[..2].try_into().unwrap());
        assert_eq!(data[2..].len(), len as usize);

        let secret = data[2..].to_vec();

        Self {
            ty,
            created,
            secret,
        }
    }
}

#[test]
fn decode_key() {
    let key_data = include_bytes!("./test.key");

    let key = CryptoKey::decode(key_data);

    panic!("{key:?}");
}
