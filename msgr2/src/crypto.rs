use crate::{CryptoKey, Encode};

pub const AUTH_MAGIC: u64 = 0xff009cad8826aa55;

pub fn encode_encrypt_enc_bl<T: Encode>(t: &T, key: &CryptoKey) -> Vec<u8> {
    let mut buffer = Vec::new();

    // Struct version
    buffer.push(1u8);
    AUTH_MAGIC.encode(&mut buffer);
    t.encode(&mut buffer);

    key.encrypt(&mut buffer);

    buffer
}

pub fn encode_encrypt<T: Encode>(t: &T, key: &CryptoKey) -> Vec<u8> {
    let encode_encrypt_bl = encode_encrypt_enc_bl(t, &key);
    let mut encoded = Vec::new();
    encode_encrypt_bl.encode(&mut encoded);
    encoded
}
