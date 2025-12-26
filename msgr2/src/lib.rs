mod connection;
mod encode;
mod entity_address;
mod entity_name;
mod entity_type;
pub mod frame;
pub mod messages;

pub use connection::{Connection, Message};
pub use encode::EncodeExt;
pub use entity_address::{EntityAddress, EntityAddressType};
pub use entity_name::EntityName;
pub use entity_type::EntityType;

mod sealed {
    pub trait Sealed {}
}
