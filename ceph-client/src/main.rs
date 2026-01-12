use std::{
    io::{Read, Write},
    net::TcpStream,
};

use msgr2::{
    Frame, Tag,
    frames::{AuthMethodCephX, AuthRequest, Banner, ClientIdent, ConMode, Hello, Keepalive},
    wire::{Completed, RxFrame, TxFrame},
};

use ceph_client::connection::{ClientConnection, Config, Message, state::Established};

use ceph_foundation::{
    CephFeatureSet, Decode, DecodeError, Encode, Timestamp, WireString,
    crypto::Key,
    entity::{EntityAddress, EntityAddressType, EntityName, EntityType},
};

fn send(frame: TxFrame<'_>, w: &mut impl std::io::Write) {
    println!("Sending: {frame:?}");
    frame.write(w).unwrap();
}

fn recv_raw<'buf, S>(
    buffer: &'buf mut Vec<u8>,
    connection: &mut ClientConnection<S>,
    mut r: &mut impl std::io::Read,
) -> RxFrame<'buf, Completed>
where
    S: Established,
{
    let rx_frame = connection.start_rx(buffer);

    let read_preamble = rx_frame.read_preamble(&mut r).unwrap();
    read_preamble.read_rest(&mut r).unwrap()
}

fn recv<'buf, S>(connection: &mut ClientConnection<S>, r: &mut impl std::io::Read) -> Message
where
    S: Established,
{
    let mut buffer = Vec::new();
    let completed = recv_raw(&mut buffer, connection, r);
    connection.finish_rx(completed).unwrap()
}

fn main() {
    let master_key = Key::decode(&mut include_bytes!("./key.bin").as_slice()).unwrap();
    let mut stream = TcpStream::connect("10.0.1.222:3300").unwrap();

    let mut config = Config::new(true);
    config.request_ticket_for(EntityType::Osd);
    let connection = ceph_client::connection::ClientConnection::new(config);

    let mut banner = connection.banner().to_bytes();

    println!("TX banner: {:?}", connection.banner());

    stream.write_all(&banner).unwrap();
    stream.read_exact(&mut banner).unwrap();

    let rx_banner = Banner::parse(&banner).unwrap();
    let mut connection = connection.recv_banner(&rx_banner).unwrap();

    println!("RX banner: {rx_banner:?}");

    let target = EntityAddress {
        ty: EntityAddressType::Msgr2,
        nonce: 0,
        address: stream.peer_addr().ok(),
    };

    let hello = Hello {
        entity_type: EntityType::Client,
        peer_address: target.clone(),
    };

    let hello_frame = connection.send_hello(&hello);
    send(hello_frame, &mut stream);

    let Message::Hello(rx_hello) = recv(&mut connection, &mut stream) else {
        panic!("Expected Hello, got something else");
    };

    println!("RX hello: {rx_hello:?}");

    let mut connection = connection.recv_hello(&rx_hello);

    let name = EntityName {
        ty: EntityType::Client,
        name: "admin".into(),
    };

    // let method = AuthMethodNone {
    //     name: EntityName {
    //         ty: EntityType::Client,
    //         name,
    //     },
    //     global_id: 1332,
    // };

    let method = AuthMethodCephX { global_id: 0, name };

    let auth_req = AuthRequest::new(method, vec![ConMode::Secure]);
    let auth_req = connection.send_req(&auth_req);
    send(auth_req, &mut stream);

    let more = match recv(&mut connection, &mut stream) {
        Message::AuthReplyMore(m) => m,
        o => panic!("Expected AuthReplyMore, got {o:?}"),
    };

    let auth_req = connection.recv_cephx_server_challenge(&master_key, &more);
    send(auth_req, &mut stream);

    let rx_auth = match recv(&mut connection, &mut stream) {
        Message::AuthDone(m) => m,
        o => panic!("Expected AuthDone, got {o:?}"),
    };

    println!("Auth rx: {rx_auth:?}");

    let mut connection = connection.recv_cephx_done(&master_key, &rx_auth).unwrap();

    println!("Recv signature");

    let Message::AuthSignature(rx_sig) = recv(&mut connection, &mut stream) else {
        panic!("Expected AuthSignature, got something else");
    };

    let signature = connection.send_signature();
    send(signature, &mut stream);

    let mut connection = connection.recv_signature(&rx_sig).unwrap();
    println!("Received signature was correct. ({rx_sig:?})");

    let ident = ClientIdent {
        addresses: vec![rx_hello.peer_address],
        target,
        gid: 14123,
        global_seq: 112123,
        supported_features: CephFeatureSet::ALL,
        required_features: CephFeatureSet::ALL,
        flags: 0,
        cookie: 1337,
    };

    let ident = connection.send_client_ident(&ident);
    send(ident, &mut stream);

    let ident_rx = match recv(&mut connection, &mut stream) {
        Message::ServerIdent(id) => id,
        Message::IdentMissingFeatures(i) => {
            panic!("Missing features: {}, {:X?}", i.features, i);
        }
        m => {
            panic!("Expected ServerIdent, got {m:?}")
        }
    };

    println!("Ident RX: {:08X?}", ident_rx);

    let mut connection = connection.recv_server_ident(&ident_rx).unwrap();

    let keepalive = Keepalive {
        timestamp: Timestamp {
            tv_sec: 123,
            tv_nsec: 456,
        },
    };

    let keepalive = connection.send(keepalive);
    send(keepalive, &mut stream);
    let rx_keepalive = recv(&mut connection, &mut stream);

    println!("Keepalive RX: {rx_keepalive:?}");

    let header = CephMessageHeader2 {
        seq: 1,
        tid: 0,
        ty: CephMessageType::Ping,
        priority: 0,
        version: 0,
        data_pre_padding_len: 0,
        data_off: 0,
        ack_seq: 0,
        flags: CephMessageHeader2Flags(0),
        compat_version: 0,
        reserved: 0,
    };

    let header = header.to_vec();

    // TODO: frame::new_message()?
    let frame = Frame::new(Tag::Message, &[&header]).unwrap();
    let frame = connection.send_raw(&frame);
    send(frame, &mut stream);

    let mut buffer = Vec::new();
    let next = recv_raw(&mut buffer, &mut connection, &mut stream);
    let next = connection.finish_rx_raw(next).unwrap();

    let ping_response = msgr2::frames::Message::decode(&next).unwrap();

    let mut reply_string = ping_response.front().unwrap();
    let reply_string: &str = WireString::decode(&mut reply_string).unwrap().into();
    println!("Ping reply JSON payload: {reply_string}");
}

pub struct CephMessageHeader2 {
    pub seq: u64,
    pub tid: u64,
    pub ty: CephMessageType,
    pub priority: u16,
    pub version: u16,
    pub data_pre_padding_len: u32,
    // TODO: automatically mask against PAGE_MASK
    pub data_off: u16,
    pub ack_seq: u64,
    pub flags: CephMessageHeader2Flags,
    pub compat_version: u16,
    pub reserved: u16,
}

ceph_foundation::write_decode_encode!(
    CephMessageHeader2 = seq
        | tid
        | ty as u16
        | priority
        | version
        | data_pre_padding_len
        | data_off
        | ack_seq
        | flags
        | compat_version
        | reserved
);

pub struct CephMessageHeader2Flags(pub u8);

impl Decode<'_> for CephMessageHeader2Flags {
    fn decode(buffer: &mut &'_ [u8]) -> Result<Self, DecodeError> {
        let (value, rest) = buffer
            .split_first()
            .ok_or_else(|| DecodeError::NotEnoughData {
                field: None,
                have: 0,
                need: 1,
            })?;

        *buffer = rest;
        Ok(Self(*value))
    }
}

impl Encode for CephMessageHeader2Flags {
    fn encode(&self, buffer: &mut Vec<u8>) {
        buffer.push(self.0)
    }
}

macro_rules! msg_type {
    ($($n:ident = $v:literal,)*)  => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        #[repr(u16)]
        pub enum CephMessageType {
            $(
                $n = $v,
            )*
        }

        impl TryFrom<u16> for CephMessageType {
            type Error = DecodeError;

            fn try_from(value: u16) -> Result<Self, Self::Error> {
                match value {
                    $(
                        $v => Ok(Self::$n),
                    )*
                    v => Err(DecodeError::unknown_value("CephMessageType", v)),
                }
            }
        }

    };
}

msg_type! {
    ShutDown = 1,
    Ping = 2,
    MonMap = 4,
    MonGetMap = 5,
    MonGetOsdMap = 6,
    MonMetadata = 7,
    StatFs = 13,
    StatFsReply = 14,
    MonSubscribe = 15,
    MonSubscribeAck = 16,
    Auth = 17,
    AuthReply = 18,
    MonGetVersion = 19,
    MonGetVersionReply = 20,
    GetPoolStats = 58,
    GetPoolStatsReply = 59,
}

impl From<&CephMessageType> for u16 {
    fn from(value: &CephMessageType) -> Self {
        *value as u16
    }
}
