pub mod crypto;
mod encdec;
pub mod entity;
mod features;
mod mon_info;
mod uuid;

pub use encdec::{Decode, DecodeError, Encode, Encoder, WireString};
pub use features::CephFeatureSet;
pub use mon_info::MonInfo;
pub use uuid::Uuid;

/// A UTC timestamp.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Timestamp {
    /// The amount of seconds since the UTC epoch.
    pub tv_sec: u32,

    /// The fractional, nanosecond amount since the UTC epoch.
    pub tv_nsec: u32,
}

impl Timestamp {
    pub fn new(sec: u32, nsec: u32) -> Self {
        Self {
            tv_sec: sec,
            tv_nsec: nsec,
        }
    }
}

write_decode_encode!(Timestamp = tv_sec | tv_nsec);

#[macro_export]
macro_rules! get_versions_and_data {
    ($ty:ty: $buffer:expr, $version:expr$(, $min_nautilus:expr)?) => {{
        let [version, compat]: [u8; 2] = $crate::Decode::decode($buffer)?;

        if compat > $version {
            return Err(DecodeError::Custom(format!(
                "Incompatible {} struct: we are at {}, received version is {} which claims compatilibity down to {}",
                stringify!($ty), $version, version, compat
            )).into());
        }

        $(
            if $version < $min_nautilus {
                return Err(DecodeError::Custom(
                    format!("Incompatible {} struct: received version {} is pre-NAUTILUS library does not support (minimum supported version: {}).", stringify!(ty), version, $min_nautilus),
                ).into());
            }
        )?

        let data: &[u8] = $crate::Decode::decode($buffer)?;

        (version, data)
    }};
}

pub struct LenWriter<'a, E>
where
    E: Encoder,
{
    encoder: &'a mut E,
    len_at: usize,
}

impl<'a, E> LenWriter<'a, E>
where
    E: Encoder,
{
    pub fn new(encoder: &'a mut E, len_at: usize) -> Self {
        Self { encoder, len_at }
    }
}

impl<E> Drop for LenWriter<'_, E>
where
    E: Encoder,
{
    fn drop(&mut self) {
        let diff = self.encoder.len().checked_sub(self.len_at).unwrap();
        let minus_len_bytes = diff.checked_sub(4).unwrap();
        let as_u32: u32 = minus_len_bytes.try_into().unwrap();
        self.encoder
            .write_at(self.len_at, as_u32.to_le_bytes().as_slice());
    }
}

impl<E> Encoder for LenWriter<'_, E>
where
    E: Encoder,
{
    fn extend_from_slice(&mut self, slice: &[u8]) {
        self.encoder.extend_from_slice(slice);
    }

    fn reserve(&mut self, len: usize) {
        self.encoder.reserve(len);
    }

    fn push(&mut self, value: u8) {
        self.encoder.push(value);
    }

    fn len(&self) -> usize {
        self.encoder.len()
    }

    fn write_at(&mut self, start: usize, data: &[u8]) {
        self.encoder.write_at(start, data);
    }
}

/// Usage: `let buffer = &mut write_versions_and_data!(buffer, version, compat);``
#[macro_export]
macro_rules! write_versions_and_data {
    ($buffer:expr, $version:expr, $compat:expr) => {{
        $buffer.extend_from_slice(&[$version, $compat]);
        let len_at = $buffer.len();
        $buffer.extend_from_slice(&0u32.to_le_bytes());
        $crate::LenWriter::new($buffer, len_at)
    }};
}
