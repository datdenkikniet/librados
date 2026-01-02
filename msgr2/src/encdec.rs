#![macro_use]

use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub enum DecodeError {
    NotEnoughData {
        field: Option<&'static str>,
        have: usize,
        need: usize,
    },
    UnexpectedVersion {
        got: u8,
        expected: RangeInclusive<u8>,
    },
    UnknownValue {
        ty: &'static str,
        value: String,
    },
    Custom(String),
}

impl DecodeError {
    pub fn for_field(self, field: &'static str) -> Self {
        match self {
            DecodeError::NotEnoughData {
                field: _,
                have,
                need,
            } => Self::NotEnoughData {
                field: Some(field),
                have,
                need,
            },
            v => v,
        }
    }
}

pub trait Decode<'a>: Sized {
    fn decode(buffer: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError>;
}

pub trait Encode {
    fn encode(&self, buffer: &mut Vec<u8>);

    fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        self.encode(&mut vec);
        vec
    }
}

fn encode_len(v: usize, buffer: &mut Vec<u8>) {
    let len = u32::try_from(v).expect("Slice length does not fit into u32");
    len.encode(buffer);
}

impl<T> Encode for &'_ T
where
    T: Encode,
{
    fn encode(&self, buffer: &mut Vec<u8>) {
        (*self).encode(buffer);
    }
}

impl Encode for [u8] {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.reserve(4 + self.len());
        encode_len(self.len(), buffer);
        buffer.extend_from_slice(self);
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(self.as_slice());
    }
}

impl<T> Encode for [T]
where
    T: Encode,
{
    fn encode(&self, buffer: &mut Vec<u8>) {
        encode_len(self.len(), buffer);
        for item in self.iter() {
            item.encode(buffer)
        }
    }
}

impl<const N: usize, T> Encode for [T; N]
where
    T: Encode,
{
    fn encode(&self, buffer: &mut Vec<u8>) {
        for item in self.iter() {
            item.encode(buffer);
        }
    }
}

macro_rules! encode_int {
    ($($int:ty),*) => {
        $(
            impl Encode for $int {
                fn encode(&self, buffer: &mut Vec<u8>) {
                    buffer.extend_from_slice(&self.to_le_bytes());
                }
            }

            impl Decode<'_> for $int {
                fn decode(buffer: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
                    if let Some((chunk, left)) = buffer.split_first_chunk() {
                        Ok(((<$int>::from_le_bytes(*chunk)), left))
                    } else {
                        Err(DecodeError::NotEnoughData { have: buffer.len(), need: <$int>::MAX.to_le_bytes().len(), field: None })
                    }
                }
            }
        )*
    };
}

// No `u8` in order to support specialized `u8`-array
// implementations.
encode_int!(u16, u32, u64, i8, i16, i32, i64);

impl<'a> Decode<'a> for &'a [u8] {
    fn decode(buffer: &'a [u8]) -> Result<(Self, &'a [u8]), DecodeError> {
        let (len, left) = u32::decode(buffer)?;

        if let Some((me, left)) = left.split_at_checked(len as usize) {
            Ok((me, left))
        } else {
            Err(DecodeError::NotEnoughData {
                have: left.len(),
                need: len as _,
                field: None,
            })
        }
    }
}

impl Decode<'_> for Vec<u8> {
    fn decode(buffer: &[u8]) -> Result<(Self, &[u8]), DecodeError> {
        let (slice, left) = <&[u8]>::decode(buffer)?;
        Ok((slice.to_vec(), left))
    }
}
