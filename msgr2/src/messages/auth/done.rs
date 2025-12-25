use crate::messages::auth::ConMode;

#[derive(Debug, Clone)]
pub struct AuthDone {
    pub global_id: u64,
    pub connection_mode: ConMode,
    pub auth_payload: Vec<u8>,
}

impl AuthDone {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 16 {
            return Err(format!(
                "Expected at least 16 bytes of auth done data, got only {}",
                data.len()
            ));
        }

        let global_id = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let connection_mode = u32::from_le_bytes(data[8..12].try_into().unwrap());

        let Ok(Ok(connection_mode)) = u8::try_from(connection_mode).map(ConMode::try_from) else {
            return Err(format!("Unknown connection mode {}", connection_mode));
        };

        let payload_bytes = u32::from_le_bytes(data[12..16].try_into().unwrap());

        if data[16..].len() as u32 != payload_bytes {
            return Err(format!(
                "Expected {} bytes of auth payload data, got only {}",
                payload_bytes,
                data[16..].len()
            ));
        }

        let auth_payload = data[16..].to_vec();

        Ok(Self {
            global_id,
            connection_mode,
            auth_payload,
        })
    }
}
