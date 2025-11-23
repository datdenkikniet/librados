#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ByteCount(u64);

impl ByteCount {
    pub const fn from_bytes(bytes: u64) -> Self {
        Self(bytes)
    }

    pub const fn from_kb(kb: u64) -> Self {
        Self(kb * 1024)
    }

    pub const fn into_kb(self) -> u64 {
        self.0 / 1024
    }

    pub const fn into_bytes(self) -> u64 {
        self.0
    }
}

impl std::ops::Add for ByteCount {
    type Output = ByteCount;

    fn add(self, rhs: Self) -> Self::Output {
        ByteCount(self.0 + rhs.0)
    }
}
