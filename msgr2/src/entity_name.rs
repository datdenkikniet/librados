use crate::EntityType;

#[derive(Debug, Clone)]
pub struct EntityName {
    pub ty: EntityType,
    pub name: String,
}

write_encdec!(EntityName = ty as u32 | name as crate::WireString);
