use crate::EntityType;

#[derive(Debug, Clone)]
pub struct EntityName {
    pub ty: EntityType,
    pub name: String,
}

write_decode_encode!(EntityName = ty as u32 | name as crate::WireString);
