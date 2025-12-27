use crate::{
    Encode,
    frame::{Frame, Msgr2Revision, Preamble, Tag},
    messages::{
        Banner, ClientIdent, Hello, IdentMissingFeatures, Keepalive, KeepaliveAck, MsgrFeatures,
        ServerIdent,
        auth::{AuthDone, AuthRequest, AuthSignature},
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

#[derive(Debug, Clone)]
pub struct Inactive {
    config: Config,
}

#[derive(Debug)]
pub struct Active {
    revision: Msgr2Revision,
}

#[derive(Clone, Debug)]
pub struct Connection<T> {
    state: T,
    buffer: Vec<u8>,
}

impl Connection<Inactive> {
    pub fn new(config: Config) -> Self {
        Self {
            state: Inactive { config },
            buffer: Vec::new(),
        }
    }

    pub fn banner(&self) -> Banner {
        let mut features = MsgrFeatures::empty();

        if self.state.config.support_rev21 {
            features.set_revision_21(true);
        }

        Banner::new(features, MsgrFeatures::empty())
    }

    pub fn recv_banner(self, banner: &Banner) -> Result<Connection<Active>, String> {
        if banner.required().compression() {
            return Err("Peer requires compression, which we do not support.".into());
        }

        let revision = if self.state.config.support_rev21 && banner.supported().revision_21() {
            Msgr2Revision::V2_1
        } else {
            Msgr2Revision::V2_0
        };

        Ok(Connection {
            state: Active { revision },
            buffer: Vec::new(),
        })
    }
}

impl Connection<Active> {
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

        let preamble = Preamble::parse(preamble_data, self.state.revision)?;

        Ok(preamble)
    }

    pub fn recv(&mut self, frame: Frame) -> Result<Message, String> {
        assert!(
            frame.segments().len() == 1,
            "Multi-segment frames not supported yet."
        );
        Ok(Message::parse(frame.tag(), frame.segments()[0])?)
    }

    pub fn send<'a, T>(&'a mut self, message: T) -> Frame<'a>
    where
        T: Into<Message>,
    {
        self.send_msg(&message.into())
    }

    pub fn send_msg<'a>(&'a mut self, message: &Message) -> Frame<'a> {
        self.buffer.clear();
        message.write_to(&mut self.buffer);

        Frame::new(message.tag(), &[&self.buffer], self.state.revision).unwrap()
    }
}

macro_rules ! message {
    ($($name:ident),*$(,)?) => {
        #[derive(Debug, Clone)]
        pub enum Message {
            $(
                $name($name),
            )*
        }

        $(
            impl From<$name> for Message {
                fn from(value: $name) -> Self {
                    Self::$name(value)
                }
            }
        )*
    }
}

message! {
    Hello,
    ClientIdent,
    ServerIdent,
    AuthRequest,
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
            _ => todo!("Unsupported tag {tag:?}"),
        }
    }
}
