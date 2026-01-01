pub mod states;

use states::{Active, Established, ExchangeHello, Inactive};

use crate::{
    Encode,
    connection::states::{Authenticating, Identifying},
    frame::{Frame, Msgr2Revision, Preamble, Tag},
    messages::{
        Banner, ClientIdent, Hello, IdentMissingFeatures, Keepalive, KeepaliveAck, MsgrFeatures,
        ServerIdent,
        auth::{
            AuthBadMethod, AuthDone, AuthReplyMore, AuthRequest, AuthRequestMore, AuthSignature,
        },
    },
};

#[derive(Debug, Clone)]
pub struct Config {
    support_rev21: bool,
}

impl Config {
    pub fn new(support_rev21: bool) -> Self {
        Self { support_rev21 }
    }
}

#[derive(Clone, Debug)]
pub struct Connection<T> {
    state: T,
    config: Config,
    buffer: Vec<u8>,
}

impl Connection<Inactive> {
    pub fn new(config: Config) -> Self {
        Self {
            state: Inactive { _reserved: () },
            config,
            buffer: Vec::new(),
        }
    }

    pub fn banner(&self) -> Banner {
        let mut features = MsgrFeatures::empty();

        if self.config.support_rev21 {
            features.set_revision_21(true);
        }

        Banner::new(features, MsgrFeatures::empty())
    }

    /// Receive the provided `banner`.
    ///
    /// This step consumes the [`Connection`]. To retry connecting, you can
    /// clone the [`Connection<Inactive>`] and re-attempt to [`recv_banner`](Connection::recv_banner).
    pub fn recv_banner(self, banner: &Banner) -> Result<Connection<ExchangeHello>, String> {
        if banner.required().compression() {
            return Err("Peer requires compression, which we do not support.".into());
        }

        let revision = if self.config.support_rev21 && banner.supported().revision_21() {
            Msgr2Revision::V2_1
        } else {
            Msgr2Revision::V2_0
        };

        Ok(Connection {
            state: ExchangeHello { revision },
            buffer: Vec::new(),
            config: self.config,
        })
    }
}

impl Connection<ExchangeHello> {
    pub fn send_hello(&mut self, hello: &Hello) -> Frame<'_> {
        self.buffer.clear();
        hello.encode(&mut self.buffer);

        Frame::new(Tag::Hello, &[&self.buffer], self.state.revision).unwrap()
    }

    pub fn recv_hello(self, _hello: &Hello) -> Connection<Authenticating> {
        Connection {
            config: self.config,
            buffer: self.buffer,
            state: Authenticating {
                revision: self.state.revision,
            },
        }
    }
}

impl Connection<Authenticating> {
    pub fn send_req(&mut self, request: &AuthRequest) -> Frame<'_> {
        self.buffer.clear();
        request.encode(&mut self.buffer);

        Frame::new(Tag::AuthRequest, &[&self.buffer], self.state.revision).unwrap()
    }

    pub fn send_more(&mut self, request: &AuthRequestMore) -> Frame<'_> {
        self.buffer.clear();
        request.encode(&mut self.buffer);

        Frame::new(Tag::AuthRequestMore, &[&self.buffer], self.state.revision).unwrap()
    }

    #[expect(unused)]
    pub fn recv_done(&mut self, done: &AuthDone) -> Frame<'_> {
        // TODO: do something with `done`.

        self.buffer.clear();

        let signature = AuthSignature {
            // TODO: actually calculate SHA256 with done info
            sha256: [0u8; _],
        };

        signature.encode(&mut self.buffer);

        Frame::new(Tag::AuthSignature, &[&self.buffer], self.state.revision).unwrap()
    }

    pub fn recv_signature(
        self,
        signature: &AuthSignature,
    ) -> Result<Connection<Identifying>, String> {
        if signature.sha256 != [0u8; _] {
            return Err("SHA256 mismatch".into());
        }

        Ok(Connection {
            state: Identifying {
                revision: self.state.revision,
            },
            config: self.config,
            buffer: self.buffer,
        })
    }
}

impl Connection<Identifying> {
    pub fn send_client_ident(&mut self, ident: &ClientIdent) -> Frame<'_> {
        self.buffer.clear();
        ident.encode(&mut self.buffer);

        Frame::new(Tag::ClientIdent, &[&self.buffer], self.state.revision).unwrap()
    }

    #[expect(unused)]
    pub fn recv_server_ident(self, ident: &ServerIdent) -> Result<Connection<Active>, String> {
        // TODO: verify details from `ident`.

        Ok(Connection {
            state: Active {
                revision: self.state.revision,
            },
            config: self.config,
            buffer: self.buffer,
        })
    }
}

impl Connection<Active> {
    pub fn send<'a, M>(&'a mut self, message: M) -> Frame<'a>
    where
        M: Into<Message>,
    {
        self.send_msg(&message.into())
    }

    pub fn send_msg<'a>(&'a mut self, message: &Message) -> Frame<'a> {
        self.buffer.clear();
        message.write_to(&mut self.buffer);

        Frame::new(message.tag(), &[&self.buffer], self.state.revision()).unwrap()
    }
}

impl<T> Connection<T>
where
    T: Established,
{
    pub fn preamble_len(&self) -> usize {
        // Fixed-size preamble for now
        crate::frame::Preamble::SERIALIZED_SIZE
    }

    pub fn recv_preamble(&mut self, preamble_data: &[u8]) -> Result<Preamble, String> {
        if preamble_data.len() != self.preamble_len() {
            return Err(format!(
                "Expected {} bytes of preamble data, got {}",
                self.preamble_len(),
                preamble_data.len()
            ));
        }

        let preamble = Preamble::parse(preamble_data, self.state.revision())?;

        Ok(preamble)
    }

    pub fn recv(&mut self, frame: Frame) -> Result<Message, String> {
        assert!(
            frame.segments().len() == 1,
            "Multi-segment frames not supported yet."
        );
        Ok(Message::parse(frame.tag(), frame.segments()[0])?)
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

    pub fn parse(tag: Tag, data: &[u8]) -> Result<Self, String> {
        match tag {
            Tag::Hello => Ok(Self::Hello(Hello::parse(&data)?)),
            Tag::ClientIdent => Ok(Self::ClientIdent(ClientIdent::parse(data)?)),
            Tag::AuthDone => Ok(Self::AuthDone(AuthDone::parse(data)?)),
            Tag::AuthSignature => Ok(Self::AuthSignature(AuthSignature::parse(data)?)),
            Tag::IdentMissingFeatures => Ok(Self::IdentMissingFeatures(
                IdentMissingFeatures::parse(data)
                    .ok_or("Incorrect amount of data for ident missing features")?,
            )),
            Tag::ServerIdent => Ok(Self::ServerIdent(ServerIdent::parse(data)?)),
            Tag::Keepalive2Ack => Ok(Self::KeepaliveAck(
                KeepaliveAck::parse(data).ok_or("Incorrect amount of data for keep alive ack")?,
            )),
            Tag::AuthBadMethod => Ok(Self::AuthBadMethod(AuthBadMethod::parse(data)?)),
            Tag::AuthRequest => Ok(Self::AuthRequest(AuthRequest::parse(data)?)),
            Tag::AuthReplyMore => Ok(Self::AuthReplyMore(AuthReplyMore::parse(data)?)),
            Tag::AuthRequestMore => todo!(),
            _ => todo!("Unsupported tag {tag:?}"),
        }
    }
}
