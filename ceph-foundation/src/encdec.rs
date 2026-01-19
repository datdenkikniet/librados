#![macro_use]

use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    ops::RangeInclusive,
};

pub fn decode_full_mut_slice(in_slice: &mut [u8]) -> Result<&mut [u8], DecodeError> {
    let have = in_slice.len();
    let (len, rest) = in_slice
        .split_first_chunk_mut()
        .ok_or(DecodeError::NotEnoughData {
            field: None,
            have,
            need: 4,
        })?;

    let len = u32::from_le_bytes(*len);

    if rest.len() != len as usize {
        return Err(DecodeError::Custom(format!(
            "Non-full slice encountered. Expected {} bytes, but had {} bytes left",
            len,
            rest.len()
        )));
    }

    Ok(rest)
}

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
        $crate::write_decode_encode!(dec_version_check($struct, $buffer): { const version $val as u8 });
        $crate::write_decode_encode!(dec($struct, $buffer): { $($($tt)*)? } with $($fields)*);
    };

    (enc($self:ident, $buffer:ident): { const version $val:literal as u8 $(| $($tt:tt)*)? }) => {
        $buffer.push($val as u8);
        $crate::write_decode_encode!(enc($self, $buffer): { $($($tt)*)? });
    };

    (dec($struct:ident, $buffer:ident): { const $val:literal as $ty:ty $(| $($tt:tt)*)? } with $($fields:ident)*) => {
        let _value = <$ty>::decode($buffer)?;
        $crate::write_decode_encode!(dec($struct, $buffer): { $($($tt)*)? } with $($fields)*);
    };

    (enc($self:ident, $buffer:ident): { const $val:literal as $ty:ty $(| $($tt:tt)*)? }) => {
        $val.encode($buffer);
        $crate::write_decode_encode!(enc($self, $buffer): { $($($tt)*)? });
    };

    (dec($struct:ident, $buffer:ident): { $field:ident $(| $($tt:tt)*)? } with $($fields:ident)*) => {
        let $field = $crate::Decode::decode($buffer).map_err(|e| e.for_field(stringify!($field)))?;
        $crate::write_decode_encode!(dec($struct, $buffer): { $($($tt)*)? } with $($fields)* $field);
    };

    (enc($self:ident, $buffer:ident): { $field:ident $(| $($tt:tt)*)? }) => {
        $self.$field.encode($buffer);
        $crate::write_decode_encode!(enc($self, $buffer): { $($($tt)*)? });
    };

    (dec($struct:ident, $buffer:ident): { $field:ident as $ty:ty $(| $($tt:tt)*)? } with $($fields:ident)*) => {
        let $field = <$ty>::decode($buffer).map_err(|e| e.for_field(stringify!($field)))?;
        let $field = TryFrom::try_from($field)?;
        $crate::write_decode_encode!(dec($struct, $buffer): { $($($tt)*)? } with $($fields)* $field);
    };

    (enc($self:ident, $buffer:ident): { $field:ident as $ty:ty $(| $($tt:tt)*)? }) => {
        <$ty>::from(&$self.$field).encode($buffer);
        $crate::write_decode_encode!(enc($self, $buffer): { $($($tt)*)? });
    };


    ($ty:ident<$lt:lifetime> = $($tt:tt)*) => {
        impl<$lt> $crate::Decode<$lt> for $ty<$lt> {
            fn decode(buffer: &mut &$lt [u8]) -> Result<Self, $crate::DecodeError> {
                $crate::write_decode_encode!(dec($ty, buffer): { $($tt)* } with);
            }
        }

        impl $crate::Encode for $ty<'_> {
            fn encode(&self, buffer: &mut impl Encoder) {
                $crate::write_decode_encode!(enc(self, buffer): { $($tt)* });
            }
        }
    };

    ($ty:ident = $($tt:tt)*) => {
        impl $crate::Decode<'_> for $ty {
            fn decode(buffer: &mut &[u8]) -> Result<Self, $crate::DecodeError> {
                $crate::write_decode_encode!(dec($ty, buffer): { $($tt)* } with);
            }
        }

        impl $crate::Encode for $ty {
            fn encode(&self, buffer: &mut impl $crate::Encoder) {
                $crate::write_decode_encode!(enc(self, buffer): { $($tt)* });
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

impl From<Infallible> for DecodeError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

/// The on-wire representation of a string.
#[derive(Default)]
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
            Err(DecodeError::Custom("Invalid string data".to_string()))
        }
    }
}

impl Encode for WireString<'_> {
    fn encode(&self, buffer: &mut impl Encoder) {
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

impl From<WireString<'_>> for String {
    fn from(value: WireString<'_>) -> Self {
        value.as_str().to_string()
    }
}

impl<'a> From<WireString<'a>> for &'a str {
    fn from(value: WireString<'a>) -> Self {
        value.0
    }
}

impl Encode for String {
    fn encode(&self, buffer: &mut impl Encoder) {
        WireString(self.as_ref()).encode(buffer);
    }
}

impl Decode<'_> for String {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, DecodeError> {
        let ws = WireString::decode(buffer)?;
        Ok(ws.as_str().to_string())
    }
}

/// A trait for decoding data from a byte buffer.
pub trait Decode<'a>: Sized {
    /// Decodes a `Self` from `buffer` (using the Ceph binary
    /// representation of `Self`), and updates `buffer` to
    /// be include the bytes left over after decoding
    /// completed.
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, DecodeError>;

    fn decode_if(cond: bool, buffer: &mut &'a [u8]) -> Result<Option<Self>, DecodeError>
    where
        Self: Default,
    {
        if cond {
            Ok(Some(Self::decode(buffer)?))
        } else {
            Ok(Default::default())
        }
    }
}

pub trait Encoder {
    fn extend_from_slice(&mut self, slice: &[u8]);
    fn reserve(&mut self, len: usize);
    fn push(&mut self, value: u8);
    fn len(&self) -> usize;
    /// Write `data` to a previously initialized sub-slice
    /// starting at `start` and ending at a `start + data.len()`
    fn write_at(&mut self, start: usize, data: &[u8]);
}

impl Encoder for Vec<u8> {
    fn extend_from_slice(&mut self, slice: &[u8]) {
        self.extend_from_slice(slice);
    }

    fn reserve(&mut self, len: usize) {
        self.reserve(len);
    }

    fn push(&mut self, value: u8) {
        self.push(value);
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn write_at(&mut self, start: usize, data: &[u8]) {
        self[start..start + data.len()].copy_from_slice(data);
    }
}

/// A trait for encoding data to a `Encoder`.
pub trait Encode {
    /// Encode the Ceph binary representation of `Self` into `buffer`.
    fn encode(&self, buffer: &mut impl Encoder);

    /// Encode `Self` to a `Vec`.
    fn to_vec(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        self.encode(&mut vec);
        vec
    }
}

fn encode_len(v: usize, buffer: &mut impl Encoder) {
    let len = u32::try_from(v).expect("Slice length does not fit into u32");
    len.encode(buffer);
}

impl<T> Encode for &'_ T
where
    T: Encode,
{
    fn encode(&self, buffer: &mut impl Encoder) {
        (*self).encode(buffer);
    }
}

impl Encode for [u8] {
    fn encode(&self, buffer: &mut impl Encoder) {
        buffer.reserve(4 + self.len());
        encode_len(self.len(), buffer);
        buffer.extend_from_slice(self);
    }
}

impl<const N: usize> Encode for [u8; N] {
    fn encode(&self, buffer: &mut impl Encoder) {
        buffer.extend_from_slice(self.as_slice());
    }
}

impl<T> Encode for [T]
where
    T: Encode,
{
    fn encode(&self, buffer: &mut impl Encoder) {
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
    fn encode(&self, buffer: &mut impl Encoder) {
        for item in self.iter() {
            item.encode(buffer);
        }
    }
}

macro_rules! encode_int {
    ($($int:ty),*) => {
        $(
            impl Encode for $int {
                fn encode(&self, buffer: &mut impl Encoder) {
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
    fn encode(&self, buffer: &mut impl Encoder) {
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

impl<K> Encode for HashSet<K>
where
    K: Encode + Eq + core::hash::Hash,
{
    fn encode(&self, buffer: &mut impl Encoder) {
        encode_len(self.len(), buffer);

        for k in self.iter() {
            k.encode(buffer);
        }
    }
}

impl<'a, K> Decode<'a> for HashSet<K>
where
    K: Decode<'a> + Eq + core::hash::Hash + 'a,
{
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, DecodeError> {
        let len = u32::decode(buffer)?;

        let mut out = HashSet::with_capacity(len as usize);

        for _ in 0..len {
            out.insert(K::decode(buffer)?);
        }

        Ok(out)
    }
}

impl<K, V> Encode for HashMap<K, V>
where
    K: Encode + Eq + core::hash::Hash,
    V: Encode,
{
    fn encode(&self, buffer: &mut impl Encoder) {
        encode_len(self.len(), buffer);

        for (k, v) in self.iter() {
            k.encode(buffer);
            v.encode(buffer);
        }
    }
}

impl<'a, K, V> Decode<'a> for HashMap<K, V>
where
    K: Decode<'a> + Eq + core::hash::Hash + 'a,
    V: Decode<'a> + 'a,
{
    fn decode(buffer: &mut &'a [u8]) -> Result<Self, DecodeError> {
        let len = u32::decode(buffer)?;

        let mut out = HashMap::with_capacity(len as usize);
        for _ in 0..len {
            let k = K::decode(buffer)?;
            let v = V::decode(buffer)?;
            out.insert(k, v);
        }

        Ok(out)
    }
}
