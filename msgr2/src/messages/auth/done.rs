use crate::messages::auth::ConMode;

#[derive(Debug, Clone)]
pub struct AuthDone {
    pub global_id: u64,
    pub connection_mode: ConMode,
    pub auth_payload: Vec<u8>,
}

write_decode_encode!(AuthDone = global_id | connection_mode | auth_payload);
