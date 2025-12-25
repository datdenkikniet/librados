use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use ceph_protocol::{
    Connection, EntityAddress, EntityAddressType, Message,
    frame::Frame,
    messages::{Banner, ClientIdent, EntityType, Features, Hello},
};

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

    let peer_address = EntityAddress {
        ty: EntityAddressType::Msgr2,
        nonce: 69,
        address: stream.peer_addr().ok(),
    };

    let hello = Hello {
        entity_type: EntityType::Client,
        peer_address: peer_address.clone(),
    };

    let hello_frame = connection.send(hello);
    hello_frame.write(&mut stream).unwrap();

    std::thread::sleep(Duration::from_millis(50));

    let mut buffer = Vec::new();

    buffer.resize(connection.preamble_len(), 0);
    stream.read_exact(&mut buffer).unwrap();

    let preamble = connection.recv_preamble(&buffer).unwrap();
    buffer.resize(preamble.data_and_epilogue_len(), 0);
    stream.read_exact(&mut buffer).unwrap();
    let frame = Frame::parse(&preamble, &buffer).unwrap();

    let Message::Hello(hello) = connection.recv(frame).unwrap() else {
        panic!("Expected hello");
    };

    println!("Received message: {:?}", hello);

    let addresses = vec![hello.peer_address];
    let client_ident = ClientIdent {
        addresses,
        target: peer_address.clone(),
        gid: 13123,
        global_seq: 123,
        supported_features: Features::empty(),
        required_features: Features::empty(),
        flags: 0,
        cookie: 4123,
    };

    let frame = connection.send(client_ident);
    frame.write(&mut stream).unwrap();

    buffer.resize(connection.preamble_len(), 0);
    stream.read_exact(&mut buffer).unwrap();

    let preamble = connection.recv_preamble(&buffer).unwrap();
    buffer.resize(preamble.data_and_epilogue_len(), 0);
    stream.read_exact(&mut buffer).unwrap();
    let frame = Frame::parse(&preamble, &buffer).unwrap();

    let _test = connection.recv(frame).unwrap();

    loop {}
}
