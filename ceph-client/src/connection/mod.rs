//! A sans-IO implementation of a `msgr2` connection, with support
//! for authentication-less and CephX connections.

mod config;
pub mod state;

use ::cephx::{CephXMessage, CephXMessageType};
use cephx::{AuthServiceTicketReply, TicketsAndConnectionSecret};
use state::{
    Active, Authenticating, Established, ExchangeHello, ExchangingSignatures, Identifying, Inactive,
};

use ceph_foundation::entity::EntityType;

use msgr2::{
    Frame, FrameEncryption, Revision, Tag,
    frames::{
        AuthBadMethod, AuthDone, AuthReplyMore, AuthRequest, AuthRequestMore, AuthSignature,
        Banner, ClientIdent, ConMode, Hello, IdentMissingFeatures, Keepalive, KeepaliveAck,
        MsgrFeatures, ServerIdent,
    },
    wire::{Completed, RxFrame, TxFrame, Unstarted},
};

pub use config::*;

use ceph_foundation::{Decode, DecodeError, Encode, Timestamp, crypto::Key};

#[derive(Clone, Debug)]
pub enum AuthError {
    Decode(DecodeError),
    NoAuthTicket,
    UnexpectedCephXMessage {
        got: CephXMessageType,
        expected: CephXMessageType,
    },
    UnexpectedData,
}

impl From<DecodeError> for AuthError {
    fn from(value: DecodeError) -> Self {
        Self::Decode(value)
    }
}

#[derive(Debug)]
pub struct ClientConnection<T> {
    state: T,
    config: Config,
    buffer: Vec<u8>,
}

impl<T> ClientConnection<T> {
    pub fn with_state<F, S>(self, state: F) -> ClientConnection<S>
    where
        F: FnOnce(T) -> S,
    {
        ClientConnection {
            state: state(self.state),
            config: self.config,
            buffer: self.buffer,
        }
    }
}

impl ClientConnection<Inactive> {
    pub fn new(config: Config) -> Self {
        let mut me = Self {
            state: Inactive {
                _reserved: (),
                rx_buf: Vec::new(),
                tx_buf: Vec::new(),
            },
            config,
            buffer: Vec::new(),
        };

        let banner = me.banner().to_bytes();
        me.state.tx_buf.extend_from_slice(banner.as_slice());

        me
    }

    pub fn banner(&self) -> Banner {
        let mut features = MsgrFeatures::empty();

        if self.config.support_rev21() {
            features.set_revision_21(true);
        }

        Banner::new(features, MsgrFeatures::empty())
    }

    /// Receive the provided `banner`.
    ///
    /// This step consumes the [`ClientConnection`]. To retry connecting, you should
    /// create a new [`ClientConnection<Inactive>`] with the same [`Confi.
    pub fn recv_banner(
        mut self,
        banner: &Banner,
    ) -> Result<ClientConnection<ExchangeHello>, String> {
        self.state
            .rx_buf
            .extend_from_slice(banner.to_bytes().as_slice());

        if banner.required().compression() {
            return Err("Peer requires compression, which we do not support".into());
        }

        if !self.banner().compatible(&banner) {
            return Err(format!(
                "Peer requires unknown msgr2 features that we do not support. Supported: {:?}, peer required: 0x{:?}",
                self.banner().supported(),
                banner.required()
            ));
        }

        let revision = if self.config.support_rev21() && banner.supported().revision_21() {
            Revision::Rev1
        } else {
            Revision::Rev0
        };

        Ok(self.with_state(|state| ExchangeHello {
            rx_buf: state.rx_buf,
            tx_buf: state.tx_buf,
            revision,
            encryption: FrameEncryption::new(),
        }))
    }
}

impl ClientConnection<ExchangeHello> {
    pub fn send_hello<'me>(&'me mut self, hello: &Hello) -> TxFrame<'me> {
        self.buffer.clear();
        hello.encode(&mut self.buffer);
        let hello = self.buffer.clone();

        let frame = Frame::new(Tag::Hello, &[&hello]).unwrap();

        frame.write(self.state.format(), &mut self.state.tx_buf);

        self.tx_frame(&frame)
    }

    pub fn recv_hello(self, _hello: &Hello) -> ClientConnection<Authenticating> {
        self.with_state(|state| Authenticating {
            revision: state.revision,
            encryption: state.encryption,
            rx_buf: state.rx_buf,
            tx_buf: state.tx_buf,
        })
    }
}

impl ClientConnection<Authenticating> {
    pub fn send_req<'me>(&'me mut self, request: &AuthRequest) -> TxFrame<'me> {
        self.buffer.clear();
        request.encode(&mut self.buffer);

        let request = self.buffer.to_vec();
        let frame = Frame::new(Tag::AuthRequest, &[&request]).unwrap();

        frame.write(self.state.format(), &mut self.state.tx_buf);

        self.tx_frame(&frame)
    }

    pub fn recv_cephx_server_challenge<'me>(
        &'me mut self,
        master_key: &Key,
        challenge: &AuthReplyMore,
    ) -> TxFrame<'me> {
        use ::cephx::*;

        let challenge = CephXServerChallenge::decode(&mut challenge.payload.as_slice()).unwrap();

        // TODO: this should be random data.
        let client_challenge = 13377;
        let key = CephXAuthenticateKey::compute(challenge.challenge, client_challenge, master_key);

        let other_keys = self
            .config
            .tickets_for()
            .iter()
            // other_keys must be non-zero, otherwise we crash the MON.
            // For now: always request ticket for auth.
            // for non-auth servers.
            // See: https://tracker.ceph.com/issues/74297
            .chain([EntityType::Auth].iter())
            .copied()
            .collect();

        let old_ticket = self.config.old_ticket().unwrap_or_default();

        let auth = CephXAuthenticate {
            client_challenge,
            key,
            old_ticket,
            other_keys,
        };

        let auth_req_more = AuthRequestMore {
            payload: CephXMessage::new(CephXMessageType::GetAuthSessionKey, auth).to_vec(),
        };

        self.buffer.clear();
        auth_req_more.encode(&mut self.buffer);

        let more = self.buffer.clone();
        let frame = Frame::new(Tag::AuthRequestMore, &[&more]).unwrap();

        frame.write(self.state.format(), &mut self.state.tx_buf);

        self.tx_frame(&frame)
    }

    pub fn recv_none_done(
        self,
        done: &AuthDone,
    ) -> Result<ClientConnection<ExchangingSignatures>, AuthError> {
        if !done.auth_payload.is_empty() {
            Err(AuthError::UnexpectedData)
        } else {
            Ok(self.with_state(|state| ExchangingSignatures {
                revision: state.revision,
                encryption: state.encryption,
                rx_buf: state.rx_buf,
                tx_buf: state.tx_buf,
                tickets: Vec::new(),
            }))
        }
    }

    pub fn recv_cephx_done(
        mut self,
        master_key: &Key,
        done: &AuthDone,
    ) -> Result<ClientConnection<ExchangingSignatures>, AuthError> {
        // TODO: save/use global ID somewhere?
        let cephx = CephXMessage::decode(&mut done.auth_payload.as_slice())?;

        if cephx.ty() != CephXMessageType::GetAuthSessionKey {
            return Err(AuthError::UnexpectedCephXMessage {
                expected: CephXMessageType::GetAuthSessionKey,
                got: cephx.ty(),
            });
        }

        let mut tickets = cephx.payload();

        let service_ticket_infos = AuthServiceTicketReply::decode(&mut tickets)?;
        assert!(tickets.is_empty());

        let TicketsAndConnectionSecret {
            tickets,
            connection_secret,
        } = service_ticket_infos.decrypt(&master_key)?;

        if done.connection_mode == ConMode::Secure {
            let encryption_key = connection_secret[00..16].try_into().unwrap();
            let rx_nonce: [u8; 12] = connection_secret[16..28].try_into().unwrap();
            let tx_nonce: [u8; 12] = connection_secret[28..40].try_into().unwrap();

            let encryption_key = Key::new(
                // TODO: probably best not to have this creation time be not completely BS
                Timestamp {
                    tv_sec: 0,
                    tv_nsec: 0,
                },
                encryption_key,
            );

            let revision = self.state.revision;
            self.state.encryption_mut().set_secret_data(
                revision,
                encryption_key,
                rx_nonce,
                tx_nonce,
            );
        }

        Ok(self.with_state(|state| ExchangingSignatures {
            tickets,
            revision: state.revision,
            encryption: state.encryption,
            rx_buf: state.rx_buf,
            tx_buf: state.tx_buf,
        }))
    }
}

impl ClientConnection<ExchangingSignatures> {
    pub fn send_signature(&mut self) -> TxFrame<'_> {
        let auth_ticket = self.state.tickets.iter().find(|t| t.ty == EntityType::Auth);

        let sha256_hmac = if let Some(session_key) =
            auth_ticket.as_ref().map(|v| &v.session_ticket.session_key)
        {
            session_key.hmac_sha256(&self.state.rx_buf)
        } else {
            [0u8; 32]
        };

        let signature = AuthSignature { sha256_hmac };

        self.buffer.clear();
        signature.encode(&mut self.buffer);

        let signature = self.buffer.clone();
        let frame = Frame::new(Tag::AuthSignature, &[&signature]).unwrap();

        self.tx_frame(&frame)
    }

    pub fn recv_signature(
        self,
        signature: &AuthSignature,
    ) -> Result<ClientConnection<Identifying>, String> {
        let auth_ticket = self.state.tickets.iter().find(|t| t.ty == EntityType::Auth);

        let valid_signature =
            if let Some(session_key) = auth_ticket.map(|v| &v.session_ticket.session_key) {
                session_key.hmac_sha256(&self.state.tx_buf)
            } else {
                [0u8; 32]
            };

        if signature.sha256_hmac != valid_signature {
            return Err("SHA256 mismatch".into());
        }

        Ok(self.with_state(|state| Identifying {
            revision: state.revision,
            encryption: state.encryption,
            tickets: state.tickets,
        }))
    }
}

impl ClientConnection<Identifying> {
    pub fn send_client_ident(&mut self, ident: &ClientIdent) -> TxFrame<'_> {
        self.buffer.clear();
        ident.encode(&mut self.buffer);

        let ident = self.buffer.clone();
        let frame = Frame::new(Tag::ClientIdent, &[&ident]).unwrap();

        self.tx_frame(&frame)
    }

    #[expect(unused)]
    pub fn recv_server_ident(
        self,
        ident: &ServerIdent,
    ) -> Result<ClientConnection<Active>, String> {
        // TODO: verify details from `ident`.

        Ok(self.with_state(|state| Active {
            revision: state.revision,
            encryption: state.encryption,
            _tickets: state.tickets,
        }))
    }
}

impl ClientConnection<Active> {
    pub fn send<'me, M>(&'me mut self, message: M) -> TxFrame<'me>
    where
        M: Into<Message>,
    {
        self.send_msg(&message.into())
    }

    pub fn send_msg<'me>(&'me mut self, message: &Message) -> TxFrame<'me> {
        self.buffer.clear();
        message.write_to(&mut self.buffer);

        let buffer = self.buffer.clone();
        let frame = Frame::new(message.tag(), &[&buffer]).unwrap();
        self.tx_frame(&frame)
    }

    pub fn send_raw<'me>(&'me mut self, frame: &Frame) -> TxFrame<'me> {
        self.tx_frame(frame)
    }
}

impl<T> ClientConnection<T>
where
    T: Established,
{
    pub fn state(&self) -> &T {
        &self.state
    }

    fn tx_frame<'me>(&'me mut self, frame: &Frame<'_>) -> TxFrame<'me> {
        self.buffer.clear();
        frame.send(
            self.state.format(),
            self.state.encryption_mut(),
            &mut self.buffer,
        )
    }

    pub fn start_rx<'enc, 'buf>(
        &'enc mut self,
        buffer: &'buf mut Vec<u8>,
    ) -> RxFrame<'buf, Unstarted<'enc>> {
        RxFrame::new(self.state.format(), self.state.encryption_mut(), buffer)
    }

    pub fn finish_rx(&mut self, frame: RxFrame<'_, Completed>) -> Result<Message, DecodeError> {
        let frame = self.finish_rx_raw(frame)?;
        Message::decode(frame.tag(), frame.segments()[0])
    }

    pub fn finish_rx_raw<'frame>(
        &mut self,
        frame: RxFrame<'frame, Completed>,
    ) -> Result<Frame<'frame>, DecodeError> {
        self.state.recv_data(frame.preamble_data());

        let (preamble, data) = frame.into_preamble_and_data();
        self.state.recv_data(data);

        Frame::decode(&preamble, data)
    }
}

macro_rules ! message {
    ($msg:ident: $($name:ident),*$(,)?) => {
        #[derive(Debug, Clone)]
        pub enum $msg {
            $(
                $name($name),
            )*
        }

        $(
            impl From<$name> for $msg {
                fn from(value: $name) -> Self {
                    Self::$name(value)
                }
            }
        )*
    }
}

message! {
    Message:
    Hello,
    ClientIdent,
    ServerIdent,
    AuthRequest,
    AuthRequestMore,
    AuthReplyMore,
    AuthBadMethod,
    AuthDone,
    AuthSignature,
    Keepalive,
    KeepaliveAck,
    IdentMissingFeatures,
}

impl Message {
    pub fn tag(&self) -> Tag {
        match self {
            Message::Hello(_) => Tag::Hello,
            Message::ClientIdent(_) => Tag::ClientIdent,
            Message::AuthRequest(_) => Tag::AuthRequest,
            Message::Keepalive(_) => Tag::Keepalive2,
            Message::AuthDone(_) => Tag::AuthDone,
            Message::AuthSignature(_) => Tag::AuthSignature,
            Message::IdentMissingFeatures(_) => Tag::IdentMissingFeatures,
            Message::ServerIdent(_) => Tag::ServerIdent,
            Message::KeepaliveAck(_) => Tag::Keepalive2Ack,
            Message::AuthBadMethod(_) => Tag::AuthBadMethod,
            Message::AuthReplyMore(_) => Tag::AuthReplyMore,
            Message::AuthRequestMore(_) => Tag::AuthRequestMore,
        }
    }

    pub fn write_to(&self, buffer: &mut Vec<u8>) {
        match self {
            Message::Hello(hello) => hello.encode(buffer),
            Message::ClientIdent(client_ident) => client_ident.encode(buffer),
            Message::AuthRequest(auth_request) => auth_request.encode(buffer),
            Message::Keepalive(keepalive) => keepalive.encode(buffer),
            Message::AuthDone(_) => todo!(),
            Message::AuthSignature(signature) => signature.encode(buffer),
            Message::IdentMissingFeatures(ident_missing_features) => {
                ident_missing_features.encode(buffer)
            }
            Message::ServerIdent(_) => todo!(),
            Message::KeepaliveAck(_) => todo!(),
            Message::AuthBadMethod(_) => todo!(),
            Message::AuthReplyMore(_) => todo!(),
            Message::AuthRequestMore(auth_request_more) => auth_request_more.encode(buffer),
        }
    }

    pub fn decode(tag: Tag, mut data: &[u8]) -> Result<Self, DecodeError> {
        match tag {
            Tag::Hello => Ok(Self::Hello(Hello::decode(&mut data)?)),
            Tag::ClientIdent => Ok(Self::ClientIdent(ClientIdent::decode(&mut data)?)),
            Tag::AuthDone => Ok(Self::AuthDone(AuthDone::decode(&mut data)?)),
            Tag::AuthSignature => Ok(Self::AuthSignature(AuthSignature::decode(&mut data)?)),
            Tag::IdentMissingFeatures => Ok(Self::IdentMissingFeatures(
                IdentMissingFeatures::decode(&mut data)?,
            )),
            Tag::ServerIdent => Ok(Self::ServerIdent(ServerIdent::decode(&mut data)?)),
            Tag::Keepalive2Ack => Ok(Self::KeepaliveAck(KeepaliveAck::decode(&mut data)?)),
            Tag::AuthBadMethod => Ok(Self::AuthBadMethod(AuthBadMethod::decode(&mut data)?)),
            Tag::AuthRequest => Ok(Self::AuthRequest(AuthRequest::decode(&mut data)?)),
            Tag::AuthReplyMore => Ok(Self::AuthReplyMore(AuthReplyMore::decode(&mut data)?)),
            Tag::AuthRequestMore => Ok(Self::AuthRequestMore(AuthRequestMore::decode(&mut data)?)),
            _ => todo!("Unsupported tag {tag:?}"),
        }
    }
}
