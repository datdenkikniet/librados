use std::collections::HashMap;

use crate::{
    Decode, Encode, WireString,
    entity::{AddrVec, EntityAddress},
};

#[derive(Debug, Clone)]
pub struct MonInfo {
    pub name: String,
    pub public_addrs: Vec<EntityAddress>,
    // Priority: lower = more important. Probably want newtype.
    pub priority: u16,
    pub weight: u16,
    pub crush_location: HashMap<String, String>,
}

impl Encode for MonInfo {
    fn encode(&self, buffer: &mut Vec<u8>) {
        todo!()
    }
}

impl Decode<'_> for MonInfo {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, crate::DecodeError> {
        let [version, compat] = <[u8; 2]>::decode(buffer)?;

        if version != 5 {
            return Err(crate::DecodeError::UnexpectedVersion {
                ty: "MonInfo",
                got: version,
                expected: 5..=5,
            });
        }

        if compat != 1 {
            return Err(crate::DecodeError::UnexpectedVersion {
                ty: "MonInfo",
                got: compat,
                expected: 1..=1,
            });
        }

        let mut data = <&[u8]>::decode(buffer)?;
        let mon_info = MonInfo9_5::decode(&mut data)?;

        Ok(mon_info.into())
    }
}

struct MonInfo9_5 {
    name: String,
    public_addrs: Vec<EntityAddress>,
    // Priority: lower = more important. Probably want newtype.
    priority: u16,
    weight: u16,
    crush_location: HashMap<String, String>,
}

write_decode_encode!(
    MonInfo9_5 = name as WireString | public_addrs as AddrVec | priority | weight | crush_location
);

impl Into<MonInfo> for MonInfo9_5 {
    fn into(self) -> MonInfo {
        MonInfo {
            name: self.name,
            public_addrs: self.public_addrs,
            // Priority: lower = more important. Probably want newtype.
            priority: self.priority,
            weight: self.weight,
            crush_location: self.crush_location,
        }
    }
}
