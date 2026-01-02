#![macro_use]

use std::ops::RangeInclusive;

macro_rules! write_encdec {
    (dec($struct:ident, $buffer:ident): { } with $($fields:ident)*) => {
        return Ok($struct { $($fields,)* });
    };

    (enc($self:ident, $buffer:ident): { }) => {};

    (dec($struct:ident, $buffer:ident): { const version $val:literal as u8 $(| $($tt:tt)*)? } with $($fields:ident)*) => {
        let Some((v, left)) = $buffer.split_first() else {
            return Err($crate::DecodeError::NotEnoughData { have: 0, need: 1, field: Some("version") })
        };

        if *v != $val {
            return Err($crate::DecodeError::UnexpectedVersion { got: *v, expected: $val..=$val })
        }

        *$buffer = left;
        write_encdec!(dec($struct, $buffer): { $($($tt)*)? } with $($fields)*);
    };

    (enc($self:ident, $buffer:ident): { const version $val:literal as u8 $(| $($tt:tt)*)? }) => {
        $buffer.push($val as u8);
        write_encdec!(enc($self, $buffer): { $($($tt)*)? });
    };

    (dec($struct:ident, $buffer:ident): { $field:ident $(| $($tt:tt)*)? } with $($fields:ident)*) => {
        #[allow(unused)]
        let $field = $crate::Decode::decode($buffer).map_err(|e| e.for_field(stringify!($field)))?;
        write_encdec!(dec($struct, $buffer): { $($($tt)*)? } with $($fields)* $field);
    };

    (enc($self:ident, $buffer:ident): { $field:ident $(| $($tt:tt)*)? }) => {
        $self.$field.encode($buffer);
        write_encdec!(enc($self, $buffer): { $($($tt)*)? });
    };

    (dec($struct:ident, $buffer:ident): { $field:ident as $ty:ty $(| $($tt:tt)*)? } with $($fields:ident)*) => {
        #[allow(unused)]
        let $field = <$ty>::decode($buffer).map_err(|e| e.for_field(stringify!($field)))?;
        let $field = TryFrom::try_from($field)?;
        write_encdec!(dec($struct, $buffer): { $($($tt)*)? } with $($fields)* $field);
    };

    (enc($self:ident, $buffer:ident): { $field:ident as $ty:ty $(| $($tt:tt)*)? }) => {
        <$ty>::from(&$self.$field).encode($buffer);
        write_encdec!(enc($self, $buffer): { $($($tt)*)? });
    };


    ($ty:ident<$lt:lifetime> = $($tt:tt)*) => {
        impl<$lt> $crate::Decode<$lt> for $ty<$lt> {
            fn decode(buffer: &mut &$lt [u8]) -> Result<Self, $crate::DecodeError> {
                write_encdec!(dec($ty, buffer): { $($tt)* } with);
            }
        }

        impl $crate::Encode for $ty<'_> {
            fn encode(&self, buffer: &mut Vec<u8>) {
                write_encdec!(enc(self, buffer): { $($tt)* });
            }
        }
    };

    ($ty:ident = $($tt:tt)*) => {
        impl $crate::Decode<'_> for $ty {
            fn decode(buffer: &mut &[u8]) -> Result<Self, $crate::DecodeError> {
                write_encdec!(dec($ty, buffer): { $($tt)* } with);
            }
        }

        impl $crate::Encode for $ty {
            fn encode(&self, buffer: &mut Vec<u8>) {
                write_encdec!(enc(self, buffer): { $($tt)* });
            }
        }
    };
}

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

pub struct WireString<'a>(&'a str);

impl<'a> Decode<'a> for WireString<'a> {
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, crate::DecodeError> {
        let slice = <&[u8]>::decode(buffer)?;

        if let Ok(str) = str::from_utf8(slice) {
            Ok(Self(str))
        } else {
            Err(DecodeError::Custom(format!("Invalid string data.")))
        }
    }
}

impl Encode for WireString<'_> {
    fn encode(&self, buffer: &mut Vec<u8>) {
        self.0.as_bytes().encode(buffer);
    }
}

impl<'a> From<&'a String> for WireString<'a> {
    fn from(value: &'a String) -> Self {
        Self(value)
    }
}

impl TryFrom<WireString<'_>> for String {
    type Error = DecodeError;

    fn try_from(value: WireString) -> Result<Self, Self::Error> {
        Ok(value.0.to_string())
    }
}

pub trait Decode<'a>: Sized {
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, DecodeError>;
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
                fn decode(buffer: &mut &[u8]) -> Result<Self, DecodeError> {
                    if let Some((chunk, left)) = buffer.split_first_chunk() {
                        *buffer = left;
                        Ok(<$int>::from_le_bytes(*chunk))
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
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, DecodeError> {
        let len = u32::decode(buffer)?;

        if let Some((me, left)) = buffer.split_at_checked(len as usize) {
            *buffer = left;
            Ok(me)
        } else {
            Err(DecodeError::NotEnoughData {
                have: buffer.len(),
                need: len as _,
                field: None,
            })
        }
    }
}

impl Decode<'_> for Vec<u8> {
    fn decode(buffer: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(<&[u8]>::decode(buffer)?.to_vec())
    }
}
