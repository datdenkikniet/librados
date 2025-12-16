use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use ceph_protocol::{
    EntityType, Hello,
    banner::Banner,
    entity_address::EntityAddress,
    frame::{Frame, Tag},
};

fn main() {
    let mut stream = TcpStream::connect("10.0.1.227:3300").unwrap();

    let banner = Banner::default();
    let mut banner_buffer = [0u8; 26];
    let tx_banner = banner.write(&mut banner_buffer).unwrap();

    stream.write_all(tx_banner).unwrap();

    stream.read_exact(&mut banner_buffer).unwrap();

    let rx_banner = Banner::parse(&banner_buffer).unwrap();

    println!("RX banner: {rx_banner:?}");

    let mut hello_buffer = [0u8; 128];
    let hello = Hello {
        entity_type: EntityType::Client,
        peer_address: EntityAddress {
            ty: ceph_protocol::entity_address::EntityAddressType::Msgr2,
            nonce: 69,
            address: stream.peer_addr().ok(),
        },
    };
    let len = hello.write(&mut hello_buffer).unwrap();

    let hello_frame = Frame::new(Tag::Hello, &[&hello_buffer[..len]]).unwrap();

    let mut frame_buffer = [0u8; 128];

    let len = hello_frame.write(&mut frame_buffer).unwrap();

    println!("{:?}", &frame_buffer[..len]);

    stream.write_all(&frame_buffer[..len]).unwrap();

    std::thread::sleep(Duration::from_millis(50));

    let hello_response = stream.read(&mut hello_buffer).unwrap();

    println!("Data: {:02X?}", &hello_buffer[..hello_response]);

    let hello_response = Frame::parse(&hello_buffer[..hello_response]).unwrap();

    println!("Hello response: {:?}", hello_response);

    let hello = Hello::parse(hello_response.segments()[0]).unwrap();
    println!("{hello:#?}");
}
