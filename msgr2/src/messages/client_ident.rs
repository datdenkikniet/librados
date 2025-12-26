use crate::{EncodeExt, entity_address::EntityAddress, messages::Features};

#[derive(Clone, Debug)]
pub struct ClientIdent {
    pub addresses: Vec<EntityAddress>,
    pub target: EntityAddress,
    pub gid: i64,
    pub global_seq: u64,
    pub supported_features: Features,
    pub required_features: Features,
    pub flags: u64,
    pub cookie: u64,
}

impl EncodeExt for ClientIdent {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.push(2u8); // Marker byte for the addrvec
        self.addresses.encode(buffer);
        self.target.encode(buffer);

        self.gid.encode(buffer);
        self.global_seq.encode(buffer);
        self.supported_features.encode(buffer);
        self.required_features.encode(buffer);
        self.flags.encode(buffer);
        self.cookie.encode(buffer);
    }
}

impl ClientIdent {
    pub(crate) fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 5 {
            return Err(format!(
                "Need at least 5 bytes for client ident, only got {}",
                data.len()
            ));
        }

        if data[0] != 2 {
            return Err(format!("Unsupported addrvec version {}", data[0]));
        }

        let len = u32::from_le_bytes([data[1], data[2], data[3], data[4]]);
        let mut left = &data[5..];
        let mut addresses = Vec::with_capacity(len as _);

        for _ in 0..len {
            let (used, address) = EntityAddress::parse(left)?;
            left = &left[used..];
            addresses.push(address);
        }

        let (used, target) = EntityAddress::parse(left)?;
        left = &left[used..];

        if left.len() < 48 {
            return Err(format!(
                "Need at least 48 leftover data bytes for client ident, got only {}",
                left.len()
            ));
        }

        let gid = i64::from_le_bytes(left[0..8].try_into().unwrap());
        let global_seq = u64::from_le_bytes(left[8..16].try_into().unwrap());
        let supported_features = u64::from_le_bytes(left[16..24].try_into().unwrap());
        let required_features = u64::from_le_bytes(left[24..32].try_into().unwrap());
        let flags = u64::from_le_bytes(left[32..40].try_into().unwrap());
        let cookie = u64::from_le_bytes(left[40..48].try_into().unwrap());

        Ok(Self {
            addresses,
            target,
            gid,
            global_seq,
            supported_features: Features(supported_features),
            required_features: Features(required_features),
            flags,
            cookie,
        })
    }
}
