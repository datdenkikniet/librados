use crate::{connection::encryption::FrameEncryption, frame::FrameFormat};

#[derive(Debug, Clone, Copy)]
pub enum Revision {
    Rev0,
    Rev1,
}

pub trait Established {
    fn format(&self) -> FrameFormat;
    fn encryption(&self) -> &FrameEncryption;
    fn encryption_mut(&mut self) -> &mut FrameEncryption;
    fn set_revision(&mut self, revision: Revision);
}

#[derive(Debug, Clone)]
pub struct Inactive {
    pub(crate) _reserved: (),
}

#[derive(Debug)]
pub struct ExchangeHello {
    pub(crate) revision: Revision,
    pub(crate) encryption: FrameEncryption,
}

#[derive(Debug)]
pub struct Authenticating {
    pub(crate) revision: Revision,
    pub(crate) encryption: FrameEncryption,
}

#[derive(Debug)]
pub struct Identifying {
    pub(crate) revision: Revision,
    pub(crate) encryption: FrameEncryption,
}

#[derive(Debug)]
pub struct Active {
    pub(crate) revision: Revision,
    pub(crate) encryption: FrameEncryption,
}

macro_rules! established {
    ($($st:ident),*) => {
        $(
            impl Established for $st {
                fn format(&self) -> FrameFormat {
                    match (self.revision, self.encryption.is_secure()) {
                        (Revision::Rev0, false) => FrameFormat::Rev0Crc,
                        (Revision::Rev1, false) => FrameFormat::Rev1Crc,
                        (Revision::Rev0, true) => FrameFormat::Rev0Secure,
                        (Revision::Rev1, true) => FrameFormat::Rev1Secure,
                    }
                }

                fn set_revision(&mut self, revision: Revision) {
                    self.revision = revision;
                }

                fn encryption(&self) -> &FrameEncryption {
                    &self.encryption
                }

                fn encryption_mut(&mut self) -> &mut FrameEncryption {
                    &mut self.encryption
                }
            }
        )*
    };
}

established!(ExchangeHello, Active, Authenticating, Identifying);
