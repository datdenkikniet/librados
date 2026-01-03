use crate::{connection::encryption::Encryption, frame::Msgr2Revision};

pub trait Established {
    fn revision(&self) -> Msgr2Revision;
    fn encryption(&self) -> &Encryption;
}

#[derive(Debug, Clone)]
pub struct Inactive {
    pub(crate) _reserved: (),
}

#[derive(Debug)]
pub struct ExchangeHello {
    pub(crate) revision: Msgr2Revision,
    pub(crate) encryption: Encryption,
}

#[derive(Debug)]
pub struct Authenticating {
    pub(crate) revision: Msgr2Revision,
    pub(crate) encryption: Encryption,
}

#[derive(Debug)]
pub struct Identifying {
    pub(crate) revision: Msgr2Revision,
    pub(crate) encryption: Encryption,
}

#[derive(Debug)]
pub struct Active {
    pub(crate) revision: Msgr2Revision,
    pub(crate) encryption: Encryption,
}

macro_rules! established {
    ($($st:ident),*) => {
        $(
            impl Established for $st {
                fn revision(&self) -> Msgr2Revision {
                    self.revision
                }

                fn encryption(&self) -> &Encryption {
                    &self.encryption
                }
            }
        )*
    };
}

established!(ExchangeHello, Active, Authenticating, Identifying);
