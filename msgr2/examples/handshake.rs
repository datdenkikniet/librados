use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use ceph_protocol::{
    Connection, EntityAddress, EntityAddressType, EntityName, EntityType, Message,
    frame::{Frame, Preamble},
    messages::{
        Banner, ClientIdent, Features, Hello, Keepalive, Timestamp,
        auth::{AuthMethodNone, AuthRequest, ConMode},
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

    let rx_hello = recv(&mut connection, &mut stream);

    println!("RX hello: {rx_hello:?}");

    let hello = Hello {
        entity_type: EntityType::Client,
        peer_address: EntityAddress {
            ty: EntityAddressType::Msgr2,
            nonce: 1412321,
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
}
