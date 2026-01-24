use std::{
    io::{Read, Write},
    net::TcpStream,
};

mod header;

use ceph_messages::{CephMessage, MonSubscribe, MonSubscribeItem};
use msgr2::{
    Frame, Tag,
    frames::{AuthMethodCephX, AuthRequest, Banner, ClientIdent, ConMode, Hello, Keepalive},
    wire::{Completed, RxFrame, TxFrame},
};

use ceph_client::connection::{ClientConnection, Config, Message, state::Established};

use ceph_foundation::{
    CephFeatureSet, Decode, Encode, Timestamp,
    crypto::Key,
    entity::{EntityAddress, EntityAddressType, EntityName, EntityType},
};

use crate::header::{CephMessageHeader2, CephMessageHeader2Flags};

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

    let mut buffer = Vec::new();

    let message = CephMessage::MonGetMap;

    let header = CephMessageHeader2 {
        seq: 1,
        transaction_id: 0,
        ty: message.identifier(),
        priority: 0,
        // TODO: this is important: header version will affect
        // encode/decode logic of sent and received messages.
        version: 0,
        data_pre_padding_len: 0,
        data_off: 0,
        ack_seq: 0,
        flags: CephMessageHeader2Flags(0),
        compat_version: None,
        reserved: 0,
    };

    let header = header.to_vec();

    // TODO: frame::new_message()?
    let frame = Frame::new(Tag::Message, &[&header]).unwrap();
    let frame = connection.send_raw(&frame);
    send(frame, &mut stream);

    let next = recv_raw(&mut buffer, &mut connection, &mut stream);
    let next = connection.finish_rx_raw(next).unwrap();
    let mon_map = CephMessage::decode_message(4, &next.segments()[1..]).unwrap();

    println!("Mon map: {mon_map:?}");

    let mon_sub = MonSubscribe {
        hostname: "desktop".to_string(),
        what: [(
            "osdmap".to_string(),
            MonSubscribeItem { start: 0, flags: 0 },
        )]
        .into_iter()
        .collect(),
    };

    let body = mon_sub.to_vec();

    let message = CephMessage::MonSubscribe(mon_sub);

    let header = CephMessageHeader2 {
        seq: 2,
        transaction_id: 0,
        ty: message.identifier(),
        priority: 0,
        version: 2,
        data_pre_padding_len: 0,
        data_off: 0,
        ack_seq: 0,
        flags: CephMessageHeader2Flags(0),
        compat_version: None,
        reserved: 0,
    };

    let header = header.to_vec();

    // TODO: frame::new_message()?
    let frame = Frame::new(Tag::Message, &[&header, &body]).unwrap();
    let frame = connection.send_raw(&frame);
    send(frame, &mut stream);

    println!("Waiting for config");

    let next = recv_raw(&mut buffer, &mut connection, &mut stream);
    let next = connection.finish_rx_raw(next).unwrap();

    let message_response = msgr2::frames::Message::from_frame(&next).unwrap();

    let header = CephMessageHeader2::decode(&mut message_response.header()).unwrap();

    println!("{header:?}");

    let message =
        ceph_messages::CephMessage::decode_message(header.ty, message_response.data_segments())
            .unwrap();

    panic!("{message:?}")
}
