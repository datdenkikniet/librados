use std::{
    collections::VecDeque,
    io::Read,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

use ceph_foundation::{
    Decode, Timestamp,
    crypto::Key,
    entity::{EntityAddress, EntityAddressType, EntityName, EntityType},
};
use msgr2::{
    Frame, FrameEncryption, FrameFormat, Tag,
    frames::{
        AuthDone, AuthMethodCephX, AuthReplyMore, AuthRequest, AuthRequestMore, AuthSignature,
        Banner, ClientIdent, ConMode, Hello, ServerIdent,
    },
    wire::RxFrame,
};

#[track_caller]
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
    // These are all secrets or settings that are simply true w.r.t. this connection,
    // or that are (eventually) negotiated in this connection.

    let client_sockaddr: SocketAddr = SocketAddrV4::new(Ipv4Addr::new(10, 0, 1, 5), 36838).into();
    let server_sockaddr: SocketAddr = SocketAddrV4::new(Ipv4Addr::new(10, 0, 1, 222), 3300).into();

    #[rustfmt::skip]
    let session_key = [108, 28, 132, 21, 133, 70, 253, 148, 37, 227, 91, 179, 135, 65, 186, 18];
    let session_key = || Key::new(Timestamp::default(), session_key);

    let c_tx_nonce = [125, 228, 109, 239, 216, 72, 244, 252, 52, 110, 241, 163];
    let s_tx_nonce = [81, 64, 21, 176, 136, 112, 18, 215, 80, 248, 20, 146];

    let mut c_tx_enc = FrameEncryption::new();
    let mut s_tx_enc = FrameEncryption::new();
    let mut fmt = FrameFormat::Rev1Crc;

    let mut s_tx_stream: VecDeque<u8> = include_bytes!("../../test-data/mon_stream.bin")
        .iter()
        .copied()
        .collect();

    let mut c_tx_stream: VecDeque<u8> = include_bytes!("../../test-data/client_stream.bin")
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

    {
        let c_hello = tx!(client);
        assert_eq!(c_hello.tag(), Tag::Hello);
        assert_eq!(c_hello.segments().len(), 1);
        assert_eq!(c_hello.segments()[0].len(), 36);

        let c_hello = Hello::decode(&mut c_hello.segments()[0].as_ref()).unwrap();

        let expected_c_hello = Hello {
            entity_type: EntityType::Client,
            peer_address: EntityAddress {
                ty: EntityAddressType::Msgr2,
                nonce: 0,
                address: Some(server_sockaddr),
            },
        };

        assert_eq!(c_hello, expected_c_hello);
    }

    {
        let s_hello = tx!(server);
        assert_eq!(s_hello.tag(), Tag::Hello);
        assert_eq!(s_hello.segments().len(), 1);
        assert_eq!(s_hello.segments()[0].len(), 36);

        let s_hello = Hello::decode(&mut s_hello.segments()[0].as_ref()).unwrap();

        let expected_s_hello = Hello {
            entity_type: EntityType::Mon,
            peer_address: EntityAddress {
                ty: EntityAddressType::Msgr2,
                nonce: 0,
                address: Some(client_sockaddr),
            },
        };

        assert_eq!(s_hello, expected_s_hello);
    }

    {
        let auth_req = tx!(client);
        assert_eq!(auth_req.tag(), Tag::AuthRequest);
        assert_eq!(auth_req.segments().len(), 1);
        assert_eq!(auth_req.segments()[0].len(), 42);

        let auth_req = AuthRequest::decode(&mut auth_req.segments()[0].as_ref()).unwrap();

        let expected_auth_req = AuthRequest::new(
            AuthMethodCephX {
                name: EntityName {
                    ty: EntityType::Client,
                    name: "admin".to_string(),
                },
                global_id: 0,
            },
            vec![ConMode::Secure, ConMode::Crc],
        );

        assert_eq!(auth_req, expected_auth_req);
    }

    {
        let auth_reply_more = tx!(server);
        assert_eq!(auth_reply_more.tag(), Tag::AuthReplyMore);
        assert_eq!(auth_reply_more.segments().len(), 1);
        assert_eq!(auth_reply_more.segments()[0].len(), 13);

        let auth_reply_more =
            AuthReplyMore::decode(&mut auth_reply_more.segments()[0].as_ref()).unwrap();

        let payload = vec![1, 56, 244, 156, 125, 244, 205, 166, 69];
        let expected_auth_reply_more = AuthReplyMore { payload };

        assert_eq!(auth_reply_more, expected_auth_reply_more);
    }

    {
        let auth_req_more = tx!(client);
        assert_eq!(auth_req_more.tag(), Tag::AuthRequestMore);
        assert_eq!(auth_req_more.segments().len(), 1);
        assert_eq!(auth_req_more.segments()[0].len(), 40);

        let auth_req_more =
            AuthRequestMore::decode(&mut auth_req_more.segments()[0].as_ref()).unwrap();

        #[rustfmt::skip]
        let payload = vec![0, 1, 3, 53, 182, 1, 161, 71, 174, 99, 173, 141, 205, 172, 6, 25, 209, 38, 243, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0];
        let expected_auth_req_more = AuthRequestMore { payload };

        assert_eq!(auth_req_more, expected_auth_req_more);
    }

    {
        let auth_done = tx!(server);
        assert_eq!(auth_done.tag(), Tag::AuthDone);
        assert_eq!(auth_done.segments().len(), 1);
        assert_eq!(auth_done.segments()[0].len(), 290);

        let auth_done = AuthDone::decode(&mut auth_done.segments()[0].as_ref()).unwrap();

        let auth_payload = vec![
            0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 32, 0, 0, 0, 1, 48, 0, 0, 0, 242, 204, 2, 44, 40, 93,
            124, 82, 245, 109, 192, 137, 79, 212, 178, 5, 10, 199, 187, 52, 120, 145, 241, 70, 170,
            134, 229, 219, 82, 137, 145, 125, 148, 249, 195, 162, 81, 6, 28, 86, 84, 166, 62, 183,
            215, 146, 248, 161, 0, 109, 0, 0, 0, 1, 4, 0, 0, 0, 0, 0, 0, 0, 96, 0, 0, 0, 148, 158,
            219, 178, 98, 233, 211, 56, 180, 26, 129, 194, 246, 144, 216, 193, 152, 231, 110, 60,
            204, 77, 66, 98, 110, 87, 98, 228, 201, 101, 164, 68, 212, 14, 66, 167, 119, 74, 132,
            205, 33, 55, 21, 33, 48, 41, 105, 165, 162, 134, 144, 175, 255, 208, 199, 191, 168,
            146, 31, 46, 120, 134, 253, 143, 232, 97, 135, 218, 192, 18, 53, 42, 41, 254, 21, 0,
            166, 248, 130, 179, 83, 182, 63, 158, 216, 95, 154, 222, 35, 194, 2, 214, 1, 140, 80,
            28, 84, 0, 0, 0, 80, 0, 0, 0, 29, 26, 22, 31, 203, 130, 34, 29, 1, 145, 116, 145, 7,
            11, 70, 62, 78, 14, 33, 22, 77, 175, 161, 108, 47, 54, 40, 108, 147, 139, 110, 3, 231,
            195, 44, 123, 229, 161, 21, 79, 96, 219, 22, 211, 121, 79, 80, 44, 92, 27, 208, 255,
            163, 46, 131, 106, 195, 199, 76, 176, 15, 114, 164, 60, 8, 55, 241, 93, 80, 155, 249,
            123, 247, 184, 18, 7, 75, 189, 65, 61, 0, 0, 0, 0,
        ];
        let expected_auth_done = AuthDone {
            global_id: 524106,
            connection_mode: ConMode::Secure,
            auth_payload,
        };

        assert_eq!(auth_done, expected_auth_done);
    }

    // After this `AuthDone`, the connection is negoatiated.
    // The details are embedded in the `auth_done` data, and
    // for sake of decoupling we do not decode them here, but
    // simply conjure them from memory.
    fmt = FrameFormat::Rev1Secure;
    c_tx_enc.set_secret_data(fmt.revision(), session_key(), c_tx_nonce, s_tx_nonce);
    s_tx_enc.set_secret_data(fmt.revision(), session_key(), s_tx_nonce, c_tx_nonce);

    {
        let client_sig = tx!(client);
        assert_eq!(client_sig.tag(), Tag::AuthSignature);
        assert_eq!(client_sig.segments().len(), 1);
        assert_eq!(client_sig.segments()[0].len(), 32);

        let sig = AuthSignature::decode(&mut client_sig.segments()[0].as_ref()).unwrap();

        #[rustfmt::skip]
        let sha256_hmac = [91, 44, 8, 74, 105, 138, 26, 102, 10, 44, 135, 36, 126, 144, 70, 105, 58, 203, 209, 225, 187, 204, 8, 46, 156, 7, 62, 190, 86, 202, 251, 121];
        let expected_sig = AuthSignature { sha256_hmac };
        assert_eq!(sig, expected_sig);
    }

    {
        let server_sig = tx!(server);
        assert_eq!(server_sig.tag(), Tag::AuthSignature);
        assert_eq!(server_sig.segments().len(), 1);
        assert_eq!(server_sig.segments()[0].len(), 32);

        let sig = AuthSignature::decode(&mut server_sig.segments()[0].as_ref()).unwrap();
        #[rustfmt::skip]
        let sha256_hmac = [13, 120, 211, 51, 110, 74, 95, 42, 17, 183, 92, 127, 51, 104, 80, 90, 20, 14, 107, 221, 206, 153, 116, 132, 94, 44, 81, 126, 162, 65, 166, 75];
        let expected_sig = AuthSignature { sha256_hmac };
        assert_eq!(sig, expected_sig);
    }

    {
        let req_compression = tx!(client);
        assert_eq!(req_compression.tag(), Tag::CompressionRequest);
        // 5 zeroes = no compression supported/requested
        assert_eq!(req_compression.segments()[0], [0u8; 5].as_slice());
    }

    {
        let comp_done = tx!(server);
        assert_eq!(comp_done.tag(), Tag::CompressionDone);
        // 5 zeroes = no compression supported/requested
        assert_eq!(comp_done.segments()[0], [0u8; 5].as_slice());
    }

    {
        let client_ident = tx!(client);
        assert_eq!(client_ident.tag(), Tag::ClientIdent);
        assert_eq!(client_ident.segments().len(), 1);
        assert_eq!(client_ident.segments()[0].len(), 123);

        let _ident = ClientIdent::decode(&mut client_ident.segments()[0].as_ref()).unwrap();
    }

    {
        let server_ident = tx!(server);
        assert_eq!(server_ident.tag(), Tag::ServerIdent);
        assert_eq!(server_ident.segments().len(), 1);
        assert_eq!(server_ident.segments()[0].len(), 123);

        let _ident = ServerIdent::decode(&mut server_ident.segments()[0].as_ref()).unwrap();
    }

    {
        let mon_get_map = tx!(client);
        assert_eq!(mon_get_map.tag(), Tag::Message);
        assert_eq!(mon_get_map.segments().len(), 1);
        assert_eq!(mon_get_map.segments()[0].len(), 41);
    }

    {
        let mon_get_map_reply = tx!(server);
        assert_eq!(mon_get_map_reply.tag(), Tag::Message);
        assert_eq!(mon_get_map_reply.segments().len(), 2);
        assert_eq!(mon_get_map_reply.segments()[0].len(), 41);
        assert_eq!(mon_get_map_reply.segments()[1].len(), 220);
    }

    {
        let subscribe = tx!(client);
        assert_eq!(subscribe.tag(), Tag::Message);
        assert_eq!(subscribe.segments().len(), 2);
        assert_eq!(subscribe.segments()[0].len(), 41);
        assert_eq!(subscribe.segments()[1].len(), 53);
    }

    {
        let sub_ack = tx!(server);
        assert_eq!(sub_ack.tag(), Tag::Message);
        assert_eq!(sub_ack.segments().len(), 2);
        assert_eq!(sub_ack.segments()[0].len(), 41);
        assert_eq!(sub_ack.segments()[1].len(), 149);
    }

    {
        let last_msg = tx!(server);
        assert_eq!(last_msg.tag(), Tag::Message);
        assert_eq!(last_msg.segments().len(), 2);
        assert_eq!(last_msg.segments()[0].len(), 41);
        assert_eq!(last_msg.segments()[1].len(), 220);
    }

    assert!(s_tx_stream.is_empty());
    assert!(s_tx_stream.is_empty());
}
