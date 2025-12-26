use crate::{EncodeExt, EntityType};

#[derive(Debug, Clone)]
pub struct EntityName {
    pub ty: EntityType,
    pub name: String,
}

impl EncodeExt for EntityName {
    fn encode(&self, buffer: &mut Vec<u8>) {
        (u8::from(self.ty) as u32).encode(buffer);
        self.name.as_bytes().encode(buffer);
    }
}
