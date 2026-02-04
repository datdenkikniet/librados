use std::collections::HashMap;

use crate::{Decode, DecodeError, Encode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
pub struct Pg {
    pub m_pool: u64,
    pub m_seed: u32,
}

impl Encode for Pg {
    fn encode(&self, buffer: &mut impl crate::Encoder) {
        // Version
        buffer.push(1u8);
        self.m_pool.encode(buffer);
        self.m_seed.encode(buffer);
        u32::MAX.encode(buffer); // Legacy field "preferred"
    }
}

impl Decode<'_> for Pg {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, crate::DecodeError> {
        let [version]: [u8; 1] = Decode::decode(buffer)?;

        if version != 1 {
            return Err(DecodeError::UnexpectedVersion {
                ty: "Pg",
                got: version,
                expected: 1..=1,
            });
        }

        let m_pool = u64::decode(buffer)?;
        let m_seed = u32::decode(buffer)?;
        let _preferred = u32::decode(buffer)?; // Legacy field

        Ok(Self { m_pool, m_seed })
    }
}

#[derive(Clone)]
pub struct PGTempMap {
    pub map: HashMap<Pg, ()>
}