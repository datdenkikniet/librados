use std::collections::HashMap;

use crate::{
    Decode, DecodeError, Encode, Encoder, Timestamp,
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
    pub time_added: Timestamp,
}

impl MonInfo {
    /// The current version for our encoding/decoding logic
    /// for this struct.
    const VERSION: u8 = 6;
}

impl Encode for MonInfo {
    fn encode(&self, buffer: &mut impl Encoder) {
        // let buffer = crate::write_versions_and_data!(buffer, Self::VERSION, 3);
        // self.name.encode(buffer);
        todo!()
    }
}

impl Decode<'_> for MonInfo {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, DecodeError> {
        let (version, mut data) = crate::get_versions_and_data!(buffer, Self::VERSION);

        if version == 1 || version == 2 {
            return Err(DecodeError::Custom(
                    "MonInfo version 1 and 2 are only supported in versions before NAUTILUS, which this library does not support."
                .to_string()));
        }

        let data = &mut data;
        let name = Decode::decode(data)?;
        let public_addrs = AddrVec::decode(data)?.try_into()?;

        // Encoded for version >= 2 (we do not support versions older than that)
        let priority = Decode::decode(data)?;

        let weight = u16::decode_if(version >= 4, buffer)?;
        let crush_location = Decode::decode_if(version >= 5, buffer)?;
        let time_added = Timestamp::decode_if(version >= 6, buffer)?;

        Ok(Self {
            name,
            public_addrs,
            priority,
            weight,
            crush_location,
            time_added,
        })
    }
}
