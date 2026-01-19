use ceph_foundation::entity::EntityType;

use crate::{CephXServiceTicket, MaybeEncryptedCephXTicketBlob};

#[derive(Debug)]
pub struct TicketsAndConnectionSecret {
    pub tickets: Vec<Ticket>,
    pub connection_secret: Vec<u8>,
}

#[derive(Debug)]
pub struct Ticket {
    pub ty: EntityType,
    pub session_ticket: CephXServiceTicket,
    pub refresh_ticket: MaybeEncryptedCephXTicketBlob,
}
