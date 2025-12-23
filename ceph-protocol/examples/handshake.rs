use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use ceph_protocol::{
    Connection, EntityType, Hello, banner::Banner, entity_address::EntityAddress, frame::Frame,
};

fn main() {
    let mut stream = TcpStream::connect("10.0.1.227:3300").unwrap();

    let mut connection = Connection::new();

    let mut banner_buffer = [0u8; 26];
    connection.banner().write(&mut banner_buffer);

    stream.write_all(&banner_buffer).unwrap();

    stream.read_exact(&mut banner_buffer).unwrap();

    let rx_banner = Banner::parse(&banner_buffer).unwrap();
    connection.recv_banner(&rx_banner).unwrap();

    println!("RX banner: {rx_banner:?}");

    let hello = Hello {
        entity_type: EntityType::Client,
        peer_address: EntityAddress {
            ty: ceph_protocol::entity_address::EntityAddressType::Msgr2,
            nonce: 69,
            address: stream.peer_addr().ok(),
        },
    };

    let hello_frame = connection.send(hello);

    hello_frame.write(&mut stream).unwrap();

    std::thread::sleep(Duration::from_millis(50));

    let mut buffer = vec![0; connection.preamble_len()];
    stream.read_exact(&mut buffer).unwrap();

    let preamble = connection.recv_preamble(&buffer).unwrap();
    buffer.resize(preamble.data_and_epilogue_len(), 0);

    stream.read_exact(&mut buffer).unwrap();

    let frame = Frame::parse(&preamble, &buffer).unwrap();

    let message = connection.recv(&frame).unwrap();

    println!("Received message: {:?}", message);
}
