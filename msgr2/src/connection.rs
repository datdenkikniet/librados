use crate::{
    EncodeExt,
    frame::{Frame, Preamble, Tag},
    messages::{
        Banner, ClientIdent, Features, Hello, Keepalive,
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
    buffer: Vec<u8>,
}

impl Connection {
    pub fn new() -> Self {
        Self {
            state: State::Inactive,
            buffer: Vec::new(),
        }
    }

    pub fn banner(&self) -> Banner {
        Banner::new(Features::empty(), Features::empty())
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

        let preamble = Preamble::parse(preamble_data)?;

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

        Frame::new(message.tag(), &[&self.buffer]).unwrap()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Hello(Hello),
    ClientIdent(ClientIdent),
    AuthRequest(AuthRequest),
    AuthDone(AuthDone),
    AuthSignature(AuthSignature),
    Keepalive(Keepalive),
}

impl From<Hello> for Message {
    fn from(value: Hello) -> Self {
        Self::Hello(value)
    }
}

impl From<ClientIdent> for Message {
    fn from(value: ClientIdent) -> Self {
        Self::ClientIdent(value)
    }
}

impl From<AuthRequest> for Message {
    fn from(value: AuthRequest) -> Self {
        Self::AuthRequest(value)
    }
}

impl From<AuthSignature> for Message {
    fn from(value: AuthSignature) -> Self {
        Self::AuthSignature(value)
    }
}

impl From<Keepalive> for Message {
    fn from(value: Keepalive) -> Self {
        Self::Keepalive(value)
    }
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
        }
    }

    pub fn parse(tag: Tag, data: &[u8]) -> Result<Self, String> {
        match tag {
            Tag::Hello => Ok(Self::Hello(Hello::parse(&data)?)),
            Tag::ClientIdent => Ok(Self::ClientIdent(ClientIdent::parse(data)?)),
            Tag::AuthDone => Ok(Self::AuthDone(AuthDone::parse(data)?)),
            Tag::AuthSignature => Ok(Self::AuthSignature(AuthSignature::parse(data)?)),
            _ => todo!("Unsupported tag {tag:?}"),
        }
    }
}
