#[derive(Debug, Clone)]
pub struct AuthReplyMore {
    pub payload: Vec<u8>,
}

impl AuthReplyMore {
    pub fn parse(buffer: &[u8]) -> Result<Self, String> {
        if let Some((len, left)) = buffer.split_first_chunk::<4>() {
            let len = u32::from_le_bytes(*len);

            if left.len() != len as usize {
                return Err(format!(
                    "Expected {len} bytes of authreplymore data, got {}",
                    left.len()
                ));
            }

            Ok(Self {
                payload: left.to_vec(),
            })
        } else {
            Err(format!(
                "Expected at least 4 bytes for authreplymore, got only {}",
                buffer.len()
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct CephXServerChallenge {
    pub challenge: u64,
}

impl CephXServerChallenge {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() != 9 {
            return Err(format!(
                "Expected 9 bytes of data for CephXServerChallenge, got {}",
                data.len()
            ));
        }

        let _version = data[0];
        let challenge = u64::from_le_bytes(data[1..].try_into().unwrap());

        Ok(Self { challenge })
    }
}
