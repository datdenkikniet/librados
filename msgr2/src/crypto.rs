use crate::{CryptoKey, Decode, DecodeError, Encode};

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

pub fn decode_decrypt_enc_bl<'a, T>(buf: &'a mut [u8], key: &CryptoKey) -> Result<T, DecodeError>
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

pub fn encode_encrypt<T: Encode>(t: &T, key: &CryptoKey) -> Vec<u8> {
    let encode_encrypt_bl = encode_encrypt_enc_bl(t, key);
    let mut encoded = Vec::new();
    encode_encrypt_bl.encode(&mut encoded);
    encoded
}
