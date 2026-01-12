use std::{collections::VecDeque, io::Read};

use msgr2::{Frame, FrameEncryption, FrameFormat, Tag, frames::Banner, wire::RxFrame};

fn read<'a>(
    format: FrameFormat,
    enc: &mut FrameEncryption,
    buffer: &'a mut Vec<u8>,
    mut read: impl Read,
) -> Frame<'a> {
    let frame = RxFrame::new(format, enc, buffer);
    let frame = frame.read_preamble(&mut read).unwrap();
    let frame = frame.read_rest(&mut read).unwrap();
    let (preamble, data) = frame.into_preamble_and_data();
    Frame::decode(&preamble, data).unwrap()
}

#[test]
fn lspools_exchange() {
    let mut c_enc = FrameEncryption::new();
    let mut s_enc = FrameEncryption::new();
    let fmt = FrameFormat::Rev1Crc;

    let mut s_tx_stream: VecDeque<u8> =
        include_bytes!("./mon_stream.bin").iter().copied().collect();

    let mut c_tx_stream: VecDeque<u8> = include_bytes!("./client_stream.bin")
        .iter()
        .copied()
        .collect();

    let mut banner = [0u8; 26];

    c_tx_stream.read_exact(&mut banner).unwrap();
    let c_banner = Banner::parse(&banner).unwrap();
    assert!(c_banner.supported().revision_21());

    s_tx_stream.read_exact(&mut banner).unwrap();
    let s_banner = Banner::parse(&banner).unwrap();
    assert!(s_banner.supported().revision_21());

    let mut buffer = Vec::new();
    macro_rules! tx {
        (client) => {
            read(fmt, &mut c_enc, &mut buffer, &mut c_tx_stream)
        };
        (server) => {
            read(fmt, &mut s_enc, &mut buffer, &mut s_tx_stream)
        };
    }

    let c_hello = tx!(client);
    assert_eq!(c_hello.tag(), Tag::Hello);

    let s_hello = tx!(server);
    assert_eq!(s_hello.tag(), Tag::Hello);

    let auth_req = tx!(client);
    assert_eq!(auth_req.tag(), Tag::AuthRequest);

    let auth_reply_more = tx!(server);
    assert_eq!(auth_reply_more.tag(), Tag::AuthReplyMore);

    let auth_req_more = tx!(client);
    assert_eq!(auth_req_more.tag(), Tag::AuthRequestMore);

    let auth_done = tx!(server);
    assert_eq!(auth_done.tag(), Tag::AuthDone);
}
