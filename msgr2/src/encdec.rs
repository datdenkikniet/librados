#![macro_use]

use std::ops::RangeInclusive;

#[macro_export]
macro_rules! write_decode_encode {
    (dec($struct:ident, $buffer:ident): { } with $($fields:ident)*) => {
        return Ok($struct { $($fields,)* });
    };

    (enc($self:ident, $buffer:ident): { }) => {};

    (dec_version_check($struct:ident, $buffer:ident): { const version $val:literal as u8 }) => {
        let Some((v, left)) = $buffer.split_first() else {
            return Err($crate::DecodeError::NotEnoughData { have: 0, need: 1, field: Some("version") })
        };

        if *v != $val {
            return Err($crate::DecodeError::UnexpectedVersion { ty: stringify!($struct), got: *v, expected: $val..=$val })
        }

        *$buffer = left;
    };

    (dec($struct:ident, $buffer:ident): { const version $val:literal as u8 $(| $($tt:tt)*)? } with $($fields:ident)*) => {
        write_decode_encode!(dec_version_check($struct, $buffer): { const version $val as u8 });
        write_decode_encode!(dec($struct, $buffer): { $($($tt)*)? } with $($fields)*);
    };

    (enc($self:ident, $buffer:ident): { const version $val:literal as u8 $(| $($tt:tt)*)? }) => {
        $buffer.push($val as u8);
        write_decode_encode!(enc($self, $buffer): { $($($tt)*)? });
    };

    (dec($struct:ident, $buffer:ident): { const $val:literal as $ty:ty $(| $($tt:tt)*)? } with $($fields:ident)*) => {
        let _value = <$ty>::decode($buffer)?;
        write_decode_encode!(dec($struct, $buffer): { $($($tt)*)? } with $($fields)*);
    };

    (enc($self:ident, $buffer:ident): { const $val:literal as $ty:ty $(| $($tt:tt)*)? }) => {
        $val.encode($buffer);
        write_decode_encode!(enc($self, $buffer): { $($($tt)*)? });
    };

    (dec($struct:ident, $buffer:ident): { $field:ident $(| $($tt:tt)*)? } with $($fields:ident)*) => {
        let $field = $crate::Decode::decode($buffer).map_err(|e| e.for_field(stringify!($field)))?;
        write_decode_encode!(dec($struct, $buffer): { $($($tt)*)? } with $($fields)* $field);
    };

    (enc($self:ident, $buffer:ident): { $field:ident $(| $($tt:tt)*)? }) => {
        $self.$field.encode($buffer);
        write_decode_encode!(enc($self, $buffer): { $($($tt)*)? });
    };

    (dec($struct:ident, $buffer:ident): { $field:ident as $ty:ty $(| $($tt:tt)*)? } with $($fields:ident)*) => {
        let $field = <$ty>::decode($buffer).map_err(|e| e.for_field(stringify!($field)))?;
        let $field = TryFrom::try_from($field)?;
        write_decode_encode!(dec($struct, $buffer): { $($($tt)*)? } with $($fields)* $field);
    };

    (enc($self:ident, $buffer:ident): { $field:ident as $ty:ty $(| $($tt:tt)*)? }) => {
        <$ty>::from(&$self.$field).encode($buffer);
        write_decode_encode!(enc($self, $buffer): { $($($tt)*)? });
    };


    ($ty:ident<$lt:lifetime> = $($tt:tt)*) => {
        impl<$lt> $crate::Decode<$lt> for $ty<$lt> {
            fn decode(buffer: &mut &$lt [u8]) -> Result<Self, $crate::DecodeError> {
                write_decode_encode!(dec($ty, buffer): { $($tt)* } with);
            }
        }

        impl $crate::Encode for $ty<'_> {
            fn encode(&self, buffer: &mut Vec<u8>) {
                write_decode_encode!(enc(self, buffer): { $($tt)* });
            }
        }
    };

    ($ty:ident = $($tt:tt)*) => {
        impl $crate::Decode<'_> for $ty {
            fn decode(buffer: &mut &[u8]) -> Result<Self, $crate::DecodeError> {
                write_decode_encode!(dec($ty, buffer): { $($tt)* } with);
            }
        }

        impl $crate::Encode for $ty {
            fn encode(&self, buffer: &mut Vec<u8>) {
                write_decode_encode!(enc(self, buffer): { $($tt)* });
            }
        }
    };
}

/// Errors that can occur while decoding a message.
#[derive(Debug, Clone)]
pub enum DecodeError {
    /// There wasn't enough data available to complete
    /// the decoding operation.
    NotEnoughData {
        /// An optional name of the field whose
        /// decoding failed.
        field: Option<&'static str>,
        /// The amount of bytes that are available.
        have: usize,
        /// The amount of bytes that are needed to complete
        /// the decoding operation.
        need: usize,
    },
    /// An unexpected version byte was found.
    UnexpectedVersion {
        /// The name of the type that is being decoded.
        ty: &'static str,
        /// The version byte that was found.
        got: u8,
        /// The version range that is supported.
        expected: RangeInclusive<u8>,
    },
    /// An unknown value (usually for enumerations) was encountered.
    UnknownValue {
        /// The name of the type that is being decoded.
        ty: &'static str,
        /// A string representation of the value that was found.
        value: String,
    },
    /// An error with a custom error message occurred.
    Custom(String),
}

impl DecodeError {
    /// Create a [`DecodeError::UnknownValue`].
    pub fn unknown_value<T: core::fmt::Display>(ty: &'static str, value: T) -> Self {
        Self::UnknownValue {
            ty,
            value: format!("{value}"),
        }
    }

    /// If this [`DecodeError`] has a `field` field in its variant, set its value
    /// to `field`.
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

/// The on-wire representation of a string.
pub struct WireString<'a>(&'a str);

impl WireString<'_> {
    pub fn as_str(&self) -> &str {
        self.0
    }
}

impl<'a> Decode<'a> for WireString<'a> {
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, crate::DecodeError> {
        let slice = <&[u8]>::decode(buffer)?;

        if let Ok(str) = str::from_utf8(slice) {
            Ok(Self(str))
        } else {
            Err(DecodeError::Custom("Invalid string data.".to_string()))
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

impl<'a> From<&'a str> for WireString<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl TryFrom<WireString<'_>> for String {
    type Error = DecodeError;

    fn try_from(value: WireString) -> Result<Self, Self::Error> {
        Ok(value.0.to_string())
    }
}

impl<'a> From<WireString<'a>> for &'a str {
    fn from(value: WireString<'a>) -> Self {
        value.0
    }
}

/// A trait for decoding data from a byte buffer.
pub trait Decode<'a>: Sized {
    /// Decodes a `Self` from `buffer` (using the Ceph binary
    /// representation of `Self`), and updates `buffer` to
    /// be include the bytes left over after decoding
    /// completed.
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, DecodeError>;
}

/// A trait for encoding data to a `Vec<u8>`.
pub trait Encode {
    /// Encode the Ceph binary representation of `Self` into `buffer`.
    fn encode(&self, buffer: &mut Vec<u8>);

    /// Encode `Self` to a `Vec`.
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

impl<'a, T> Decode<'a> for Vec<T>
where
    T: Decode<'a> + 'a,
{
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, DecodeError> {
        let len = u32::decode(buffer)? as usize;
        let mut res = Vec::with_capacity(len);

        for _ in 0..len {
            res.push(T::decode(buffer)?);
        }

        Ok(res)
    }
}

impl<const N: usize> Decode<'_> for [u8; N] {
    fn decode(buffer: &mut &[u8]) -> Result<Self, DecodeError> {
        if let Some((chunk, left)) = buffer.split_first_chunk() {
            *buffer = left;
            Ok(*chunk)
        } else {
            Err(DecodeError::NotEnoughData {
                field: None,
                have: buffer.len(),
                need: N,
            })
        }
    }
}

impl Encode for bool {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.push(*self as u8)
    }
}

impl Decode<'_> for bool {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, DecodeError> {
        let Some((v, left)) = buffer.split_first() else {
            return Err(DecodeError::NotEnoughData {
                have: 0,
                need: 1,
                field: Some("version"),
            });
        };

        *buffer = left;

        Ok(*v != 0)
    }
}
