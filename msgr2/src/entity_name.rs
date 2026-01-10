use crate::EntityType;

/// An entity name.
#[derive(Debug, Clone)]
pub struct EntityName {
    /// The entity type.
    pub ty: EntityType,
    /// The name of the entity.
    pub name: String,
}

ceph_foundation::write_decode_encode!(EntityName = ty as u32 | name as ceph_foundation::WireString);
