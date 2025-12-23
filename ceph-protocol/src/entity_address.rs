use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use nix::libc::{AF_INET, AF_INET6};

#[derive(Debug, Clone, PartialEq)]
pub struct EntityAddress {
    pub ty: EntityAddressType,
    pub nonce: u32,
    pub address: Option<SocketAddr>,
}

impl EntityAddress {
    pub fn write(&self, buffer: &mut Vec<u8>) {
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
        buffer.push(1);

        // Version and compat version
        buffer.push(1);
        buffer.push(1);

        let data_len = len - 3 - 4;
        buffer.extend_from_slice(&(data_len as u32).to_le_bytes());
        buffer.extend_from_slice(&(self.ty as u32).to_le_bytes());
        buffer.extend_from_slice(&self.nonce.to_le_bytes());
        buffer.extend_from_slice(&address_len.to_le_bytes());

        match self.address {
            Some(SocketAddr::V4(v4_addr)) => {
                buffer.extend_from_slice(&(AF_INET as u16).to_le_bytes());
                buffer.extend_from_slice(&v4_addr.port().to_be_bytes());
                buffer.extend_from_slice(v4_addr.ip().octets().as_slice());
            }
            Some(SocketAddr::V6(v6_addr)) => {
                buffer.extend_from_slice(&(AF_INET6 as u16).to_le_bytes());
                buffer.extend_from_slice(&v6_addr.port().to_le_bytes());
                buffer.extend_from_slice(&v6_addr.flowinfo().to_le_bytes());
                buffer.extend_from_slice(v6_addr.ip().octets().as_slice());
                buffer.extend_from_slice(&v6_addr.scope_id().to_le_bytes());
            }
            None => {}
        };
    }

    pub fn parse(data: &[u8]) -> Result<Self, String> {
        // TODO: length check!

        let address_version = data[0];
        // 1 = has feature addr2 (is this msgr2?)
        assert_eq!(address_version, 1);

        let encoding_version = data[1];
        assert_eq!(encoding_version, 1);

        let encoding_compat = data[2];
        assert_eq!(encoding_compat, 1);

        let len = u32::from_le_bytes(data[3..7].try_into().unwrap());
        assert_eq!(data[7..].len(), len as _);

        let ty = u32::from_le_bytes(data[7..11].try_into().unwrap());

        let ty =
            EntityAddressType::try_from(ty).map_err(|_| format!("Unknown entity type {}", ty))?;

        let nonce = u32::from_le_bytes(data[11..15].try_into().unwrap());

        let address_len = u32::from_le_bytes(data[15..19].try_into().unwrap()) as usize;

        let address = if address_len != 0 {
            let family = u16::from_le_bytes(data[19..21].try_into().unwrap());
            let input = data.as_ptr_range();
            let data = &data[21..21 + (address_len - 2)];
            assert_eq!(data.as_ptr_range().end, input.end);

            if family as i32 == AF_INET {
                let port = u16::from_be_bytes(data[..2].try_into().unwrap());
                let address = Ipv4Addr::new(data[2], data[3], data[4], data[5]);

                Some(SocketAddr::V4(SocketAddrV4::new(address, port)))
            } else if family as i32 == AF_INET6 {
                let port = u16::from_be_bytes(data[..2].try_into().unwrap());
                let flowinfo = u32::from_le_bytes(data[2..6].try_into().unwrap());
                let address = Ipv6Addr::from_octets(data[6..22].try_into().unwrap());
                let scope_id = u32::from_le_bytes(data[22..26].try_into().unwrap());

                Some(SocketAddr::V6(SocketAddrV6::new(
                    address, port, flowinfo, scope_id,
                )))
            } else {
                return Err(format!("Unknown address family {}", family));
            }
        } else {
            None
        };

        Ok(Self { nonce, ty, address })
    }
}

/// The type of entity that we are talking
/// to (at the communication level).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum EntityAddressType {
    None = 0,
    Legacy = 1,
    Msgr2 = 2,
    Any = 3,
    Cidr = 4,
}

impl TryFrom<u32> for EntityAddressType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let res = match value {
            0 => Self::None,
            1 => Self::Legacy,
            2 => Self::Msgr2,
            3 => Self::Any,
            4 => Self::Cidr,
            _ => return Err(()),
        };

        Ok(res)
    }
}
impl TryFrom<u8> for EntityAddressType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(value as u32)
    }
}
