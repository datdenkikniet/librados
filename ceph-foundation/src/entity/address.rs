use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

const AF_INET: u16 = 2;
const AF_INET6: u16 = 10;

use crate::{Decode, DecodeError, Encode, Encoder};

#[derive(Clone, Copy)]
struct SocketAddressWrapper(SocketAddr);

impl SocketAddressWrapper {
    pub fn encoded_len(&self) -> u32 {
        match self.0 {
            SocketAddr::V4(_) => 6,
            SocketAddr::V6(_) => 26,
        }
    }
}

impl Encode for SocketAddressWrapper {
    fn encode(&self, buffer: &mut impl Encoder) {
        match &self.0 {
            SocketAddr::V4(v4_addr) => {
                AF_INET.encode(buffer);
                // IMPORTANT: port is encoded big-endian, so swap()
                v4_addr.port().swap_bytes().encode(buffer);
                v4_addr.ip().octets().encode(buffer);
            }
            SocketAddr::V6(v6_addr) => {
                AF_INET6.encode(buffer);
                // IMPORTANT: port is encoded big-endian, so swap()
                v6_addr.port().swap_bytes().encode(buffer);
                v6_addr.flowinfo().encode(buffer);
                v6_addr.ip().octets().encode(buffer);
                v6_addr.scope_id().encode(buffer);
            }
        };
    }
}

impl Decode<'_> for SocketAddressWrapper {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, DecodeError> {
        let family = u16::decode(buffer)?;

        if family == AF_INET {
            let port = u16::decode(buffer)?.swap_bytes();
            let data: [u8; 4] = Decode::decode(buffer)?;
            let address = Ipv4Addr::from_octets(data);
            Ok(Self(SocketAddr::V4(SocketAddrV4::new(address, port))))
        } else if family == AF_INET6 {
            let port = u16::decode(buffer)?.swap_bytes();
            let flowinfo = u32::decode(buffer)?;
            let address_data: [u8; 16] = Decode::decode(buffer)?;
            let address = Ipv6Addr::from_octets(address_data);
            let scope_id = u32::decode(buffer)?;

            Ok(Self(SocketAddr::V6(SocketAddrV6::new(
                address, port, flowinfo, scope_id,
            ))))
        } else {
            return Err(DecodeError::unknown_value("AddressFamily", family));
        }
    }
}

/// An entity address.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntityAddress {
    /// The type of the entity address.
    pub ty: EntityAddressType,
    /// A nonce associated with the entity.
    pub nonce: u32,
    /// The socket address of the entity.
    pub address: Option<SocketAddr>,
}

impl Encode for EntityAddress {
    fn encode(&self, buffer: &mut impl Encoder) {
        let address = self.address.map(SocketAddressWrapper);
        let address_len = address.map(|v| 2 + v.encoded_len()).unwrap_or(0) as u32;

        let len = 3 // Version bytes
            + 4 // Len
            + 4 // Type
            + 4 // Nonce
            + 4 // address len
            + address_len;

        // Address version for >= NAUTILUS
        // Version and compat version
        [1u8, 1, 1].encode(buffer);

        let data_len = len - 3 - 4;
        data_len.encode(buffer);
        u32::from(self.ty).encode(buffer);
        self.nonce.encode(buffer);
        address_len.encode(buffer);

        if let Some(address) = address {
            address.encode(buffer);
        }
    }
}

impl Decode<'_> for EntityAddress {
    fn decode(buffer: &mut &[u8]) -> Result<Self, DecodeError> {
        let versions: [u8; 3] = Decode::decode(buffer).map_err(|e| e.for_field("versions"))?;

        if versions[0] != 1 {
            return Err(DecodeError::UnexpectedVersion {
                ty: "EntityAddress.version",
                got: versions[0],
                expected: 1..=1,
            });
        }

        if versions[1] != 1 {
            return Err(DecodeError::UnexpectedVersion {
                ty: "EntityAddress.encoding_version",
                got: versions[0],
                expected: 1..=1,
            });
        }

        if versions[2] != 1 {
            return Err(DecodeError::UnexpectedVersion {
                ty: "EntityAddress.encoding_compat",
                got: versions[0],
                expected: 1..=1,
            });
        }

        let additional_data = <&[u8]>::decode(buffer)?;
        let mut additional_data = additional_data;

        let ty = u32::decode(&mut additional_data)?.try_into()?;

        let nonce = u32::decode(&mut additional_data)?;

        let address_data = <&[u8]>::decode(&mut additional_data)?;
        let mut address_data = address_data;

        let address = if !address_data.is_empty() {
            Some(SocketAddressWrapper::decode(&mut address_data)?.0)
        } else {
            None
        };

        Ok(Self { nonce, ty, address })
    }
}

/// The type of entity that we are talking
/// to (at the communication level).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
#[expect(missing_docs)]
pub enum EntityAddressType {
    None = 0,
    Legacy = 1,
    Msgr2 = 2,
    Any = 3,
    Cidr = 4,
}

impl From<EntityAddressType> for u32 {
    fn from(value: EntityAddressType) -> Self {
        value as u32
    }
}

impl TryFrom<u32> for EntityAddressType {
    type Error = DecodeError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let res = match value {
            0 => Self::None,
            1 => Self::Legacy,
            2 => Self::Msgr2,
            3 => Self::Any,
            4 => Self::Cidr,
            _ => {
                return Err(DecodeError::unknown_value("EntityAddressType", value));
            }
        };

        Ok(res)
    }
}
impl TryFrom<u8> for EntityAddressType {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(value as u32)
    }
}

#[test]
fn round_trip_v4() {
    let v4 = EntityAddress {
        ty: EntityAddressType::Legacy,
        nonce: 42,
        address: Some(SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::from_octets([1, 2, 3, 4]),
            1337,
        ))),
    };

    let encoded = v4.to_vec();
    let decoded = EntityAddress::decode(&mut encoded.as_slice()).unwrap();
    assert_eq!(v4, decoded);
}

#[test]
fn round_trip_v6() {
    let v6 = EntityAddress {
        ty: EntityAddressType::Legacy,
        nonce: 42,
        address: Some(SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::from_octets([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            1337,
            9001,
            3999,
        ))),
    };

    let encoded = v6.to_vec();
    let decoded = EntityAddress::decode(&mut encoded.as_slice()).unwrap();
    assert_eq!(v6, decoded);
}

#[test]
fn round_trip_none() {
    let v6 = EntityAddress {
        ty: EntityAddressType::Legacy,
        nonce: 42,
        address: None,
    };

    let encoded = v6.to_vec();
    let decoded = EntityAddress::decode(&mut encoded.as_slice()).unwrap();
    assert_eq!(v6, decoded);
}

#[test]
fn sanity_check_v6() {
    let data = &[
        1, 1, 1, 40, 0, 0, 0, 1, 0, 0, 0, 42, 0, 0, 0, 28, 0, 0, 0, 10, 0, 5, 57, 41, 35, 0, 0, 1,
        2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 159, 15, 0, 0,
    ];

    let expected = EntityAddress {
        ty: EntityAddressType::Legacy,
        nonce: 42,
        address: Some(SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::from_octets([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            1337,
            9001,
            3999,
        ))),
    };
    let decoded = EntityAddress::decode(&mut data.as_slice()).unwrap();

    assert_eq!(expected, decoded);
}
