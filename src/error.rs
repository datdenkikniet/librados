pub type Result<T> = std::result::Result<T, RadosError>;

#[must_use]
pub fn maybe_err(value: i32) -> Result<()> {
    (value >= 0).then_some(()).ok_or(value.into())
}

#[derive(Debug, Clone, PartialEq)]
pub enum RadosError {
    Noent,
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
