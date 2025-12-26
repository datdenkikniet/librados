use crate::{
    Encode,
    frame::{Frame, Msgr2Revision, Preamble, Tag},
    messages::{
        Banner, ClientIdent, Hello, IdentMissingFeatures, Keepalive, KeepaliveAck, MsgrFeatures,
        ServerIdent,
        auth::{AuthDone, AuthRequest, AuthSignature},
    },
};

enum State {
    Inactive,
    Active,
}

impl State {
    fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }

    fn is_inactive(&self) -> bool {
        matches!(self, Self::Inactive)
    }
}

pub struct Connection {
    state: State,
    revision: Msgr2Revision,
    buffer: Vec<u8>,
}

impl Connection {
    pub fn new() -> Self {
        Self {
            state: State::Inactive,
            revision: Msgr2Revision::V2_0,
            buffer: Vec::new(),
        }
    }

    pub fn banner(&self) -> Banner {
        Banner::new(MsgrFeatures::empty(), MsgrFeatures::empty())
    }

    pub fn msgr2_revision(&self) -> Msgr2Revision {
        self.revision
    }

    pub fn preamble_len(&self) -> usize {
        // Fixed-size preamble for now
        crate::frame::Preamble::SERIALIZED_SIZE
    }

    pub fn recv_banner(&mut self, banner: &Banner) -> Result<(), String> {
        assert!(self.state.is_inactive());

        if banner.required().compression() {
            return Err("Peer requires compression, which we do not support.".into());
        }

        if banner.required().revision_21() {
            return Err("Peer requires msgr revision 2.1, which we do not support".into());
        }

        self.state = State::Active;

        Ok(())
    }

    pub fn recv_preamble(&mut self, preamble_data: &[u8]) -> Result<Preamble, String> {
        assert!(self.state.is_active());

        if preamble_data.len() != self.preamble_len() {
            return Err(format!(
                "Expected {} bytes of preamble data, got {}",
                self.preamble_len(),
                preamble_data.len()
            ));
        }

        let preamble = Preamble::parse(preamble_data, self.revision)?;

        Ok(preamble)
    }

    pub fn recv(&mut self, frame: Frame) -> Result<Message, String> {
        assert!(self.state.is_active());
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
        assert!(self.state.is_active());

        self.buffer.clear();
        message.write_to(&mut self.buffer);

        Frame::new(message.tag(), &[&self.buffer], self.revision).unwrap()
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
