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

write_decode_encode!(Timestamp = tv_sec | tv_nsec);

#[macro_export]
macro_rules! get_versions_and_data {
    ($buffer:expr, $version:expr) => {{
        let [version, compat]: [u8; 2] = $crate::Decode::decode($buffer)?;

        if compat > $version {
            return Err(DecodeError::Custom(format!(
                "Incompatible MonInfo struct: we are at {}, received version is {} which claims compatilibity down to {}",
                $version, version, compat
            )));
        }

        let data: &[u8] = $crate::Decode::decode($buffer)?;

        (version, data)
    }};
}

struct LenWriter<'a> {
    buffer: &'a mut Vec<u8>,
    len_at: usize,
}

impl Drop for LenWriter<'_> {
    fn drop(&mut self) {
        let diff = self.buffer.len().checked_sub(self.len_at).unwrap();
        let minus_len_bytes = diff.checked_sub(4).unwrap();
        let as_u32: u32 = minus_len_bytes.try_into().unwrap();
        self.buffer[self.len_at..self.len_at + 4].copy_from_slice(&as_u32.to_le_bytes());
    }
}

impl core::borrow::Borrow<Vec<u8>> for LenWriter<'_> {
    fn borrow(&self) -> &Vec<u8> {
        &self.buffer
    }
}

impl core::borrow::BorrowMut<Vec<u8>> for LenWriter<'_> {
    fn borrow_mut(&mut self) -> &mut Vec<u8> {
        &mut self.buffer
    }
}

#[macro_export]
macro_rules! write_versions_and_data {
    ($buffer:expr, $version:expr, $compat:expr) => {{
        $buffer.extend_from_slice(&[$version, $compat]);
        let len_at = $buffer.len();
        $buffer.extend_from_slice(&0u32.to_le_bytes());
        $crate::LenWriter {
            buffer: $buffer,
            len_at,
        }
    }};
}
