/// A generic result type, containing a value `T` or a `RadosError`.
pub type Result<T> = std::result::Result<T, RadosError>;

#[must_use]
pub fn maybe_err(value: i32) -> Result<()> {
    (value >= 0).then_some(()).ok_or(value.into())
}

#[must_use]
pub fn maybe_err_or_val(value: i32) -> Result<u32> {
    u32::try_from(value).map_err(|_| value.into())
}

/// A rados error.
#[derive(Debug, Clone, PartialEq)]
pub enum RadosError {
    /// Entry could not be found.
    ///
    /// This error is returned when the entry that an operation
    /// requires (i.e. a pool, or an object) does not exist.
    Noent,
    /// Unknown.
    ///
    /// A generic error that catches any unknown error codes.
    Unknown(i32),
}

impl From<i32> for RadosError {
    fn from(value: i32) -> Self {
        let value = value.abs();

        match value {
            2 => Self::Noent,
            v => Self::Unknown(v),
        }
    }
}
