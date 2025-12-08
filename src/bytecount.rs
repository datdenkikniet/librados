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

impl core::fmt::Display for ByteCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = self.0;

        const KIB_BOUND: u64 = 1024;
        const MIB_BOUND: u64 = 512 * KIB_BOUND;
        const GIB_BOUND: u64 = 1024 * MIB_BOUND;
        const TIB_BOUND: u64 = 1024 * GIB_BOUND;

        let (pow, sf) = match bytes {
            0..KIB_BOUND => (0, "b"),
            KIB_BOUND..MIB_BOUND => (1, "KiB"),
            KIB_BOUND..GIB_BOUND => (2, "MiB"),
            GIB_BOUND..TIB_BOUND => (3, "GiB"),
            _ => (4, "TiB"),
        };

        let div = 1024u64.pow(pow);

        write!(f, "{:.01} {}", bytes as f64 / div as f64, sf)
    }
}
