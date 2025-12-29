use crate::messages::auth::{AuthMethod, ConMode};

#[derive(Debug, Clone)]
pub struct AuthBadMethod {
    pub method: AuthMethod,
    pub result: u32,
    pub allowed_methods: Vec<AuthMethod>,
    pub allowed_modes: Vec<ConMode>,
}

impl AuthBadMethod {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 16 {
            return Err(format!(
                "Need at least 16 bytes for auth bad method, got only {}",
                data.len()
            ));
        }

        let method = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let Ok(method) = AuthMethod::try_from(method) else {
            return Err(format!("Unknown auth method {}", method));
        };

        let result = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);

        let mut allowed_method_count = u32::from_le_bytes(data[8..12].try_into().unwrap());
        let mut allowed_methods = Vec::with_capacity(allowed_method_count as usize);

        let mut left = &data[12..];

        while allowed_method_count > 0 {
            if left.len() < 4 {
                return Err(format!("Expected more allowed methods"));
            }

            let method = u32::from_le_bytes(left[..4].try_into().unwrap());
            let Ok(method) = AuthMethod::try_from(method) else {
                return Err(format!("Unknown auth method {}", method));
            };
            allowed_methods.push(method);
            left = &left[4..];
            allowed_method_count -= 1;
        }

        let mut allowed_mode_count = u32::from_le_bytes(data[..4].try_into().unwrap());
        let mut allowed_modes = Vec::with_capacity(allowed_mode_count as usize);

        let mut left = &left[4..];

        while allowed_mode_count > 0 {
            if left.len() < 4 {
                return Err(format!("Expected more allowed methods"));
            }

            let mode = u32::from_le_bytes(left[..4].try_into().unwrap());
            let Ok(mode) = ConMode::try_from(mode) else {
                return Err(format!("Unknown con mode {}", mode));
            };

            allowed_modes.push(mode);
            left = &left[4..];
            allowed_mode_count -= 1;
        }

        Ok(Self {
            method,
            result,
            allowed_methods,
            allowed_modes,
        })
    }
}
