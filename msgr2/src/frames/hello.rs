use ceph_foundation::{
    Decode, DecodeError, Encode,
    entity::{EntityAddress, EntityType},
};

/// A basic hello message, relaying information about
/// the entity that sends it.
#[derive(Debug, Clone)]
pub struct Hello {
    /// The type of the entity.
    pub entity_type: EntityType,
    /// The address of the peer the entity is communicating
    /// with.
    ///
    /// This is the address of the peer, i.e. the address of
    /// the `mon` or `osd` that the entity is talking to.
    pub peer_address: EntityAddress,
}

impl Encode for Hello {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.entity_type.into());
        self.peer_address.encode(buffer);
    }
}

impl Decode<'_> for Hello {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, DecodeError> {
        let (entity_type, rest) = buffer.split_first().ok_or(DecodeError::NotEnoughData {
            field: Some("entity_type"),
            have: buffer.len(),
            need: 1,
        })?;

        let entity_type = EntityType::try_from(*entity_type)?;

        *buffer = rest;

        let address = EntityAddress::decode(buffer)?;

        Ok(Self {
            entity_type,
            peer_address: address,
        })
    }
}

#[test]
fn valid_hello() {
    let data = &[
        1, 1, 1, 1, 28, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 2, 0, 138, 144, 10, 0, 1, 5,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let _hello = Hello::decode(&mut &data[..]).unwrap();
}

#[test]
fn round_trip() {
    use std::net::*;

    let hello = Hello {
        entity_type: EntityType::Client,
        peer_address: EntityAddress {
            ty: ceph_foundation::entity::EntityAddressType::Msgr2,
            nonce: 1337,
            address: Some(SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::new(10, 0, 1, 5),
                1337,
            ))),
        },
    };

    let mut hello_buffer = Vec::new();
    hello.encode(&mut hello_buffer);

    println!("{:?}", hello_buffer);

    let output_hello = Hello::decode(&mut hello_buffer.as_slice()).unwrap();

    assert_eq!(output_hello.entity_type, hello.entity_type);
    assert_eq!(output_hello.peer_address, hello.peer_address);
}
