use std::{collections::VecDeque, io::Read};

use ceph_client::{CephMessageHeader2, connection::auth::AuthServiceTicketReply};
use ceph_foundation::{
    Decode,
    crypto::{Key, decode_decrypt_enc_bl},
    entity::EntityType,
};
use cephx::{CephXMessage, CephXMessageType, CephXServiceTicket};
use msgr2::{
    Frame, FrameEncryption, FrameFormat, Tag,
    frames::{AuthDone, AuthSignature, Banner},
    wire::RxFrame,
};

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
    let master_key = Key::decode(&mut include_bytes!("./key.bin").as_slice()).unwrap();
    let mut c_tx_enc = FrameEncryption::new();
    let mut s_tx_enc = FrameEncryption::new();
    let mut fmt = FrameFormat::Rev1Crc;

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
            read(fmt, &mut c_tx_enc, &mut buffer, &mut c_tx_stream)
        };
        (server) => {
            read(fmt, &mut s_tx_enc, &mut buffer, &mut s_tx_stream)
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

    let auth_done = AuthDone::decode(&mut auth_done.segments().next().unwrap()).unwrap();

    let (key, s_tx_nonce, c_tx_nonce) = {
        // TODO: save/use global ID somewhere?
        let cephx = CephXMessage::decode(&mut auth_done.auth_payload.as_slice()).unwrap();
        assert_eq!(cephx.ty(), CephXMessageType::GetAuthSessionKey);
        let mut tickets = cephx.payload();
        let mut service_ticket_infos = AuthServiceTicketReply::decode(&mut tickets).unwrap();

        let info = &mut service_ticket_infos.service_ticket_reply.tickets[0];
        assert_eq!(info.ty, EntityType::Auth);

        let service_session_ticket: CephXServiceTicket =
            decode_decrypt_enc_bl(&mut info.encrypted_session_ticket, &master_key).unwrap();

        // TODO: do something with this (refresh?) ticket
        let _service_refresh_ticket = &info.refresh_ticket;

        let encrypted = service_ticket_infos.connection_secret.clone();
        let mut encrypted = <&[u8]>::decode(&mut encrypted.as_slice()).unwrap().to_vec();
        let secret: &[u8] =
            decode_decrypt_enc_bl(&mut encrypted, &service_session_ticket.session_key).unwrap();

        let key: [u8; 16] = secret[00..16].try_into().unwrap();
        let rx_nonce: [u8; 12] = secret[16..28].try_into().unwrap();
        let tx_nonce: [u8; 12] = secret[28..40].try_into().unwrap();

        (key, rx_nonce, tx_nonce)
    };

    fmt = FrameFormat::Rev1Secure;

    c_tx_enc.set_secret_data(
        fmt.revision(),
        Key::new(Default::default(), key),
        c_tx_nonce,
        s_tx_nonce,
    );

    s_tx_enc.set_secret_data(
        fmt.revision(),
        Key::new(Default::default(), key),
        s_tx_nonce,
        c_tx_nonce,
    );

    let client_sig = tx!(client);
    assert_eq!(client_sig.tag(), Tag::AuthSignature);
    let sig = AuthSignature::decode(&mut client_sig.segments().next().unwrap().as_ref()).unwrap();

    #[rustfmt::skip]
    let expected = [91, 44, 8, 74, 105, 138, 26, 102, 10, 44, 135, 36, 126, 144, 70, 105, 58, 203, 209, 225, 187, 204, 8, 46, 156, 7, 62, 190, 86, 202, 251, 121];
    assert_eq!(sig.sha256_hmac, expected);

    let server_sig = tx!(server);
    assert_eq!(server_sig.tag(), Tag::AuthSignature);
    let sig = AuthSignature::decode(&mut server_sig.segments().next().unwrap().as_ref()).unwrap();
    #[rustfmt::skip]
    let expected = [13, 120, 211, 51, 110, 74, 95, 42, 17, 183, 92, 127, 51, 104, 80, 90, 20, 14, 107, 221, 206, 153, 116, 132, 94, 44, 81, 126, 162, 65, 166, 75];
    assert_eq!(sig.sha256_hmac, expected);

    let req_compression = tx!(client);
    assert_eq!(req_compression.tag(), Tag::CompressionRequest);
    // 5 zeroes = no compression supported/requested
    assert_eq!(req_compression.segments().next(), Some([0u8; 5].as_slice()));

    let comp_done = tx!(server);
    assert_eq!(comp_done.tag(), Tag::CompressionDone);
    // 5 zeroes = no compression supported/requested
    assert_eq!(comp_done.segments().next(), Some([0u8; 5].as_slice()));

    let client_ident = tx!(client);
    assert_eq!(client_ident.tag(), Tag::ClientIdent);

    let server_ident = tx!(server);
    assert_eq!(server_ident.tag(), Tag::ServerIdent);

    let next = tx!(client);
    let header = CephMessageHeader2::decode(&mut next.segments().next().unwrap()).unwrap();

    panic!("{header:?}");
}
