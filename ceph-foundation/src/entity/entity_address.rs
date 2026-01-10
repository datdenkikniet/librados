use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

const AF_INET: u16 = 2;
const AF_INET6: u16 = 10;

use crate::{Decode, DecodeError, Encode};

/// An entity address.
#[derive(Debug, Clone, PartialEq)]
pub struct EntityAddress {
    /// The type of the entity address.
    pub ty: EntityAddressType,
    /// A nonce associated with the entity.
    pub nonce: u32,
    /// The socket address of the entity.
    pub address: Option<SocketAddr>,
}

impl Encode for EntityAddress {
    fn encode(&self, buffer: &mut Vec<u8>) {
        let address_len = self
            .address
            .map(|v| {
                let addr_len = match v {
                    SocketAddr::V4(_) => 6,
                    SocketAddr::V6(_) => 26,
                };

                2 + addr_len
            })
            .unwrap_or(0) as u32;

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
        (self.ty as u32).encode(buffer);
        self.nonce.encode(buffer);
        address_len.encode(buffer);

        match self.address {
            Some(SocketAddr::V4(v4_addr)) => {
                (AF_INET as u16).encode(buffer);

                // TODO: how to deal with be-encoding port?
                buffer.extend_from_slice(&v4_addr.port().to_be_bytes());

                v4_addr.ip().octets().encode(buffer);
            }
            Some(SocketAddr::V6(v6_addr)) => {
                (AF_INET6 as u16).encode(buffer);

                // TODO: how to deal with be-encoding port?
                buffer.extend_from_slice(&v6_addr.port().to_be_bytes());

                v6_addr.flowinfo().encode(buffer);
                v6_addr.ip().octets().encode(buffer);
                v6_addr.scope_id().encode(buffer);
            }
            None => {}
        };
    }
}

impl Decode<'_> for EntityAddress {
    fn decode(buffer: &mut &[u8]) -> Result<Self, DecodeError> {
        // TODO: length check!

        let mut used = 1;
        let address_version = buffer[0];
        // 1 = has feature addr2 (is this msgr2?)
        assert_eq!(address_version, 1);

        used += 1;
        let encoding_version = buffer[1];
        assert_eq!(encoding_version, 1);

        used += 1;
        let encoding_compat = buffer[2];
        assert_eq!(encoding_compat, 1);

        let len = u32::from_le_bytes(buffer[3..7].try_into().unwrap());
        assert!(buffer[7..].len() >= len as _);
        used += 4 + len;

        let ty = u32::from_le_bytes(buffer[7..11].try_into().unwrap());

        let ty = EntityAddressType::try_from(ty)?;

        let nonce = u32::from_le_bytes(buffer[11..15].try_into().unwrap());

        let address_len = u32::from_le_bytes(buffer[15..19].try_into().unwrap()) as usize;

        let address = if address_len != 0 {
            let family = u16::from_le_bytes(buffer[19..21].try_into().unwrap());
            let data = &buffer[21..21 + (address_len - 2)];

            if family == AF_INET {
                let port = u16::from_be_bytes(data[..2].try_into().unwrap());
                let address = Ipv4Addr::new(data[2], data[3], data[4], data[5]);

                Some(SocketAddr::V4(SocketAddrV4::new(address, port)))
            } else if family == AF_INET6 {
                let port = u16::from_be_bytes(data[..2].try_into().unwrap());
                let flowinfo = u32::from_le_bytes(data[2..6].try_into().unwrap());
                let address = Ipv6Addr::from_octets(data[6..22].try_into().unwrap());
                let scope_id = u32::from_le_bytes(data[22..26].try_into().unwrap());

                Some(SocketAddr::V6(SocketAddrV6::new(
                    address, port, flowinfo, scope_id,
                )))
            } else {
                return Err(DecodeError::unknown_value("AddressFamily", family));
            }
        } else {
            None
        };

        *buffer = &buffer[used as _..];

        Ok(Self { nonce, ty, address })
    }
}

/// The type of entity that we are talking
/// to (at the communication level).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[expect(missing_docs)]
pub enum EntityAddressType {
    None = 0,
    Legacy = 1,
    Msgr2 = 2,
    Any = 3,
    Cidr = 4,
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
