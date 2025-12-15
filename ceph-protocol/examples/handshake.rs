use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

use ceph_protocol::{
    banner::Banner,
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
    let hello_frame = Frame::new(Tag::Hello, &[&[]]).unwrap();

    let len = hello_frame.write(&mut hello_buffer).unwrap();

    stream.write_all(&hello_buffer[..len]).unwrap();

    std::thread::sleep(Duration::from_millis(50));

    let hello_response = stream.read(&mut hello_buffer).unwrap();

    println!("Data: {:02X?}", &hello_buffer[..hello_response]);

    let hello_response = Frame::parse(&hello_buffer[..hello_response]).unwrap();

    println!("Hello response: {:?}", hello_response);
}
