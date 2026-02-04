use std::collections::HashMap;

use crate::{
    Decode, DecodeError, Encode, Encoder, Timestamp, WireString,
    entity::{AddrVec, EntityAddress},
};

#[derive(Debug, Clone, PartialEq)]
pub struct MonInfo {
    pub name: String,
    pub public_addrs: Vec<EntityAddress>,
    // Priority: lower = more important. Probably want newtype.
    pub priority: u16,
    pub weight: u16,
    pub crush_location: HashMap<String, String>,
    pub time_added: Option<Timestamp>,
}

impl MonInfo {
    /// The current version for our encoding/decoding logic
    /// for this struct.
    const VERSION: u8 = 6;
}

impl Encode for MonInfo {
    fn encode(&self, buffer: &mut impl Encoder) {
        let buffer = &mut crate::write_versions_and_data!(buffer, Self::VERSION, 3);
        self.name.encode(buffer);
        self.public_addrs.encode(buffer);
        self.priority.encode(buffer);
        self.weight.encode(buffer);
        self.crush_location.encode(buffer);
        self.time_added.clone().unwrap_or_default().encode(buffer);
    }
}

impl Decode<'_> for MonInfo {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, DecodeError> {
        let (version, mut buffer) =
            crate::get_versions_and_data!(MonInfo: buffer, Self::VERSION, 3);
        let buffer = &mut buffer;

        let name = WireString::decode(buffer)?.into();
        let public_addrs = AddrVec::decode(buffer)?.try_into()?;

        // Encoded for version >= 2 (we do not support versions older than that)
        let priority = Decode::decode(buffer)?;

        let weight = u16::decode_if(version >= 4, buffer)?.unwrap_or_default();
        let crush_location = Decode::decode_if(version >= 5, buffer)?.unwrap_or_default();
        let time_added = Decode::decode_if(version >= 6, buffer)?;

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
