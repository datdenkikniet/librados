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
        )*
    };
}

// No `u8` in order to support specialized `u8`-array
// implementations.
encode_int!(u16, u32, u64, i8, i16, i32, i64);
