use crate::DecodeError;

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

impl From<EntityType> for u32 {
    fn from(value: EntityType) -> Self {
        u8::from(value) as u32
    }
}

impl From<&EntityType> for u32 {
    fn from(value: &EntityType) -> Self {
        From::from(*value)
    }
}

impl From<&EntityType> for u8 {
    fn from(value: &EntityType) -> Self {
        u8::from(*value)
    }
}

impl TryFrom<u8> for EntityType {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let res = match value {
            0x01 => Self::Mon,
            0x02 => Self::Mds,
            0x04 => Self::Osd,
            0x08 => Self::Client,
            0x10 => Self::Mgr,
            0x20 => Self::Auth,
            0xFF => Self::Any,
            _ => {
                return Err(DecodeError::unknown_value("EntityType", value));
            }
        };

        Ok(res)
    }
}

impl TryFrom<u32> for EntityType {
    type Error = DecodeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match u8::try_from(value) {
            Ok(v) => EntityType::try_from(v),
            Err(_) => Err(DecodeError::unknown_value("EntityType", value)),
        }
    }
}
