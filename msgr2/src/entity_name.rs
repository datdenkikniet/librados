use crate::EntityType;

#[derive(Debug, Clone)]
pub struct EntityName {
    pub ty: EntityType,
    pub name: String,
}

impl EntityName {
    pub fn write_to(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(&(u8::from(self.ty) as u32).to_le_bytes());

        buffer.extend_from_slice(&(self.name.len() as u32).to_le_bytes());
        buffer.extend_from_slice(self.name.as_bytes());
    }
}
