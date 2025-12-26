use std::{
    io::{Read, Write},
    net::TcpStream,
};

use ceph_protocol::{
    CephFeatureSet, Connection, EntityAddress, EntityAddressType, EntityName, EntityType, Message,
    frame::Frame,
    messages::{
        Banner, ClientIdent, Hello, Keepalive, Timestamp,
        auth::{AuthMethodNone, AuthRequest, AuthSignature, ConMode},
    },
};

fn send<T>(connection: &mut Connection, w: &mut impl std::io::Write, msg: T)
where
    T: Into<Message>,
{
    let frame = connection.send(msg);
    let to_send = frame.to_vec();

    println!(
        "Sending: {:?}, {}, {}",
        frame,
        frame.segments()[0].len(),
        to_send.len()
    );

    w.write_all(&to_send).unwrap();
    w.flush().unwrap();
}

fn recv(connection: &mut Connection, r: &mut impl std::io::Read) -> Message {
    let mut buffer = Vec::new();
    buffer.resize(connection.preamble_len(), 0);
    let len = r.read(&mut buffer).unwrap();
    println!("Read {} bytes of preamble data.", len);

    if len != connection.preamble_len() {
        panic!("{:?}", &buffer[..len]);
    }

    let preamble = connection.recv_preamble(&buffer).unwrap();
    buffer.resize(preamble.data_and_epilogue_len(), 0);
    r.read(&mut buffer).unwrap();

    let frame = Frame::parse(&preamble, &buffer).unwrap();

    connection.recv(frame).unwrap()
}

fn main() {
    let mut stream = TcpStream::connect("10.0.1.227:3300").unwrap();

    let mut connection = Connection::new();

    let mut banner_buffer = [0u8; Banner::SERIALIZED_SIZE];
    connection.banner().write(&mut banner_buffer);

    stream.write_all(&banner_buffer).unwrap();
    stream.read_exact(&mut banner_buffer).unwrap();

    let rx_banner = Banner::parse(&banner_buffer).unwrap();
    connection.recv_banner(&rx_banner).unwrap();

    println!("RX banner: {rx_banner:?}");

    let Message::Hello(rx_hello) = recv(&mut connection, &mut stream) else {
        panic!("Expected Hello, got something else");
    };

    println!("RX hello: {rx_hello:?}");

    let hello = Hello {
        entity_type: EntityType::Client,
        peer_address: EntityAddress {
            ty: EntityAddressType::Msgr2,
            nonce: rx_hello.peer_address.nonce,
            address: stream.peer_addr().ok(),
        },
    };

    send(&mut connection, &mut stream, hello.clone());

    let method = AuthMethodNone {
        name: EntityName {
            ty: EntityType::Client,
            name: "client.1332".into(),
        },
        global_id: 1332,
    };
    let auth_req = AuthRequest::new(method, vec![ConMode::Crc]);

    send(&mut connection, &mut stream, auth_req);
    let rx_auth = recv(&mut connection, &mut stream);

    println!("Auth rx: {rx_auth:?}");

    let rx_sig = recv(&mut connection, &mut stream);
    println!("Signature rx: {rx_sig:?}");

    send(
        &mut connection,
        &mut stream,
        AuthSignature { sha256: [0u8; _] },
    );

    let target = EntityAddress {
        ty: EntityAddressType::Msgr2,
        nonce: 0,
        address: stream.peer_addr().ok(),
    };

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

    send(&mut connection, &mut stream, ident);
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

    let keepalive = Keepalive {
        timestamp: Timestamp {
            tv_sec: 123,
            tv_nsec: 456,
        },
    };

    send(&mut connection, &mut stream, keepalive);
    let rx_keepalive = recv(&mut connection, &mut stream);

    println!("Keepalive RX: {rx_keepalive:?}");
}
