//! The different states that a connection can be in.

use cephx::CephXServiceTicket;
use msgr2::frame::{FrameEncryption, FrameFormat, Revision};

/// A connection state that is capable of receiving [`RxFrame`][0]s and
/// transmitting [`TxFrame`][1]s
///
/// [0]: crate::frame::RxFrame
/// [1]: crate::frame::TxFrame
#[doc(hidden)]
pub trait Established {
    /// Get the format of frames exchanged over this connection.
    fn format(&self) -> FrameFormat;

    fn encryption(&self) -> &FrameEncryption;
    fn encryption_mut(&mut self) -> &mut FrameEncryption;
    fn set_revision(&mut self, revision: Revision);

    fn recv_data(&mut self, _data: &[u8]) {}
}

/// An inactive connection.
#[derive(Debug, Clone)]
pub struct Inactive {
    pub(crate) _reserved: (),
    pub(crate) rx_buf: Vec<u8>,
    pub(crate) tx_buf: Vec<u8>,
}

/// A connection where the server-client pair is exchanging
/// [`Hello`](crate::messages::Hello) messages.
#[derive(Debug)]
pub struct ExchangeHello {
    pub(crate) revision: Revision,
    pub(crate) encryption: FrameEncryption,
    pub(crate) rx_buf: Vec<u8>,
    pub(crate) tx_buf: Vec<u8>,
}

/// A connection that is in the process of authorizing and
/// authenticating.
#[derive(Debug)]
pub struct Authenticating {
    pub(crate) revision: Revision,
    pub(crate) encryption: FrameEncryption,
    pub(crate) rx_buf: Vec<u8>,
    pub(crate) tx_buf: Vec<u8>,
}

/// A connection that is in the process of exchanging
/// connection signatures.
#[derive(Debug)]
pub struct ExchangingSignatures {
    pub(crate) revision: Revision,
    pub(crate) encryption: FrameEncryption,
    pub(crate) rx_buf: Vec<u8>,
    pub(crate) tx_buf: Vec<u8>,
    pub(crate) auth_ticket: Option<CephXServiceTicket>,
}

/// A connection where the clien-server pair is exchanging
/// [`ClientIdent`](crate::messages::ClientIdent) and
/// [`ServerIdent`](crate::messages::ServerIdent) messages.
#[derive(Debug)]
pub struct Identifying {
    pub(crate) revision: Revision,
    pub(crate) encryption: FrameEncryption,
    pub(crate) auth_ticket: Option<CephXServiceTicket>,
}

/// An active connection, sending and receiving upper-protocol
/// `Message` data.
#[derive(Debug)]
pub struct Active {
    pub(crate) revision: Revision,
    pub(crate) encryption: FrameEncryption,
    pub(crate) _auth_ticket: Option<CephXServiceTicket>,
}

macro_rules! established {
    ($($st:ident $($rx_buf:ident)?),*) => {
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

                $(
                    fn recv_data(&mut self, data: &[u8]) {
                        self.$rx_buf.extend_from_slice(data);
                    }
                )?
            }
        )*
    };
}

established!(ExchangeHello rx_buf, Authenticating rx_buf, ExchangingSignatures, Identifying, Active);
