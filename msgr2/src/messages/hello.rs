use crate::entity_address::EntityAddress;

#[derive(Debug, Clone)]
pub struct Hello {
    /// The type of entity we are communicating with.
    pub entity_type: EntityType,
    /// The peer address that the entity we are communicating
    /// with observes us to have.
    pub peer_address: EntityAddress,
}

impl Hello {
    pub(crate) fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.entity_type.into());
        self.peer_address.write(buffer);
    }

    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let entity_type = EntityType::try_from(data[0])
            .map_err(|_| format!("Unknown entity type {}", data[0]))?;
        let (_, address) = EntityAddress::parse(&data[1..])?;

        Ok(Self {
            entity_type,
            peer_address: address,
        })
    }
}

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

#[test]
fn valid_hello() {
    let data = &[
        1, 1, 1, 1, 28, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 2, 0, 138, 144, 10, 0, 1, 5,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let _hello = Hello::parse(&data[..]).unwrap();
}

#[test]
fn round_trip() {
    use std::net::*;

    let hello = Hello {
        entity_type: EntityType::Client,
        peer_address: EntityAddress {
            ty: crate::entity_address::EntityAddressType::Msgr2,
            nonce: 1337,
            address: Some(SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::new(10, 0, 1, 5),
                1337,
            ))),
        },
    };

    let mut hello_buffer = Vec::new();
    hello.write_to(&mut hello_buffer);

    println!("{:?}", hello_buffer);

    let output_hello = Hello::parse(&hello_buffer).unwrap();

    assert_eq!(output_hello.entity_type, hello.entity_type);
    assert_eq!(output_hello.peer_address, hello.peer_address);
}
