use crate::Encode;

/// The type of entity we are talking to (MON, MDS, OSD, CLIENT or MGR).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Mon,
    Mds,
    Osd,
    Client,
    Mgr,
    Auth,
    Any,
}

impl From<EntityType> for u8 {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::Mon => 0x01,
            EntityType::Mds => 0x02,
            EntityType::Osd => 0x04,
            EntityType::Client => 0x08,
            EntityType::Mgr => 0x10,
            EntityType::Auth => 0x20,
            EntityType::Any => 0xFF,
        }
    }
}

impl TryFrom<u8> for EntityType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let res = match value {
            0x01 => Self::Mon,
            0x02 => Self::Mds,
            0x04 => Self::Osd,
            0x08 => Self::Client,
            0x10 => Self::Mgr,
            0x20 => Self::Auth,
            0xFF => Self::Any,
            _ => return Err(()),
        };

        Ok(res)
    }
}

impl Encode for EntityType {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.push(u8::from(*self))
    }
}
