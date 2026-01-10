use ceph_foundation::entity::EntityType;
use cephx::CephXTicketBlob;

#[derive(Debug, Clone)]
pub struct Config {
    support_rev21: bool,
    request_tickets_for: Vec<EntityType>,
    old_ticket: Option<CephXTicketBlob>,
}

impl Config {
    pub fn new(support_rev21: bool) -> Self {
        Self {
            support_rev21,
            request_tickets_for: Vec::new(),
            old_ticket: None,
        }
    }

    pub fn with_old_ticket(&mut self, ticket: CephXTicketBlob) {
        self.old_ticket = Some(ticket);
    }

    pub fn old_ticket(&self) -> Option<CephXTicketBlob> {
        self.old_ticket.clone()
    }

    pub fn support_rev21(&self) -> bool {
        self.support_rev21
    }

    pub fn request_ticket_for(&mut self, entity: EntityType) {
        if !self.request_tickets_for.contains(&entity) {
            self.request_tickets_for.push(entity);
        }
    }

    pub fn tickets_for(&self) -> &[EntityType] {
        &self.request_tickets_for
    }
}
