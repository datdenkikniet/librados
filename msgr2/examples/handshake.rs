use std::{
    io::{Read, Write},
    net::TcpStream,
};

use ceph_protocol::{
    CephFeatureSet, CryptoKey, Decode, Encode, EntityAddress, EntityAddressType, EntityName,
    EntityType, Timestamp,
    connection::{Config, Connection, Message, state::Established},
    frame::Frame,
    messages::{
        Banner, ClientIdent, Hello, Keepalive,
        auth::{
            AuthMethodCephX, AuthRequest, AuthRequestMore, AuthSignature, CephXServerChallenge,
            ConMode,
        },
        cephx::{
            AuthServiceTicketInfos, CephXAuthenticate, CephXAuthenticateKey, CephXMessage,
            CephXMessageType, CephXServiceTicket, CephXTicketBlob,
        },
    },
};

fn send(frame: Frame<'_>, w: &mut impl std::io::Write) {
    let to_send = frame.to_vec();

    println!(
        "Sending: {:?}, {}, {}",
        frame,
        frame.segments().next().unwrap().len(),
        to_send.len()
    );

    w.write_all(&to_send).unwrap();
    w.flush().unwrap();
}

fn recv<S>(connection: &mut Connection<S>, r: &mut impl std::io::Read) -> Message
where
    S: Established,
{
    let mut buffer = Vec::new();
    buffer.resize(connection.preamble_len(), 0);
    r.read_exact(&mut buffer).unwrap();
    println!("Read {} bytes of preamble data.", buffer.len());

    if buffer.len() != connection.preamble_len() {
        unreachable!()
    }

    let mut preamble = connection.recv_preamble(&buffer).unwrap();
    buffer.resize(preamble.data_and_epilogue_len(), 0);

    if !buffer.is_empty() {
        r.read_exact(&mut buffer).unwrap();
    }

    connection.recv(&mut preamble, &buffer).unwrap()
}

fn main() {
    let master_key = CryptoKey::decode(&mut include_bytes!("./key.bin").as_slice()).unwrap();
    let mut stream = TcpStream::connect("10.0.1.222:3300").unwrap();

    let config = Config::new(true);
    let mut connection = ceph_protocol::connection::Connection::new(config);

    let mut banner = connection.banner().to_bytes();

    println!("TX banner: {:?}", connection.banner());

    stream.write_all(&banner).unwrap();
    stream.read_exact(&mut banner).unwrap();

    let rx_banner = Banner::parse(&banner).unwrap();
    let mut connection = connection.recv_banner(&rx_banner).unwrap();

    println!("RX banner: {rx_banner:?}");

    let hello = Hello {
        entity_type: EntityType::Client,
        peer_address: EntityAddress {
            ty: EntityAddressType::Msgr2,
            nonce: 118844,
            address: stream.peer_addr().ok(),
        },
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

    let auth_req = AuthRequest::new(method, vec![ConMode::Secure, ConMode::Crc]);
    let auth_req = connection.send_req(&auth_req);
    send(auth_req, &mut stream);

    let more = match recv(&mut connection, &mut stream) {
        Message::AuthReplyMore(m) => m,
        o => panic!("Expected AuthReplyMore, got {o:?}"),
    };

    let challenge = CephXServerChallenge::decode(&mut more.payload.as_slice()).unwrap();

    println!("Server challenge: {challenge:?}");

    let client_challenge = 13377;
    let key = CephXAuthenticateKey::compute(challenge.challenge, client_challenge, &master_key);
    let auth = CephXAuthenticate {
        client_challenge,
        key,
        old_ticket: CephXTicketBlob {
            secret_id: 0,
            blob: Vec::new(),
        },
        other_keys: u8::from(EntityType::Mon) as u32 | u8::from(EntityType::Osd) as u32,
    };

    let auth_req_more = AuthRequestMore {
        payload: CephXMessage::new(CephXMessageType::GetAuthSessionKey, auth).to_vec(),
    };

    let auth_req = connection.send_more(&auth_req_more);
    send(auth_req, &mut stream);

    let rx_auth = match recv(&mut connection, &mut stream) {
        Message::AuthDone(m) => m,
        o => panic!("Expected AuthDone, got {o:?}"),
    };

    println!("Auth rx: {rx_auth:?}");

    let auth_done = CephXMessage::decode(&mut rx_auth.auth_payload.as_slice()).unwrap();

    if auth_done.ty() != CephXMessageType::GetAuthSessionKey {
        panic!(
            "Expected CephXMessage of type GetAuthSessionKey, got {:?}",
            auth_done.ty()
        );
    }

    let mut tickets = auth_done.payload();

    let mut service_ticket_infos = AuthServiceTicketInfos::decode(&mut tickets).unwrap();
    assert!(tickets.is_empty());

    let mut auth_service_ticket = None;
    let mut auth_service_secret = None;

    for info in &mut service_ticket_infos.info_list {
        println!("Ticket entity: {:?}", info.service_id);
        println!("Additional ticket data: {:?}", info.refresh_ticket);

        let service_session_ticket: CephXServiceTicket =
            ceph_protocol::crypto::decode_decrypt_enc_bl(
                &mut info.encrypted_session_ticket,
                &master_key,
            )
            .unwrap();

        let _service_refresh_ticket = info.refresh_ticket.as_unencrypted_mut().unwrap();

        let encrypted = service_ticket_infos.connection_secret.clone();
        let mut encrypted = <&[u8]>::decode(&mut encrypted.as_slice()).unwrap().to_vec();
        let connection_secret: &[u8] = ceph_protocol::crypto::decode_decrypt_enc_bl(
            &mut encrypted,
            &service_session_ticket.session_key,
        )
        .unwrap();

        if info.service_id == EntityType::Auth {
            auth_service_ticket = Some(service_session_ticket);
            auth_service_secret = Some(connection_secret.to_vec());
        }

        println!("Connection secret len: {}", connection_secret.len());
    }

    println!("CBL len: {}", service_ticket_infos.connection_secret.len());
    println!("Extra len: {}", service_ticket_infos.extra.len());

    let Some(auth_service_ticket) = auth_service_ticket.take() else {
        panic!("Did not get service ticket for auth service");
    };

    let Some(auth_service_secret) = auth_service_secret.take() else {
        panic!("Did not get service secret for auth service");
    };
    // We are encrypted here: let's deal with that

    let encryption_key = auth_service_secret[00..16].try_into().unwrap();
    let rx_nonce: [u8; 12] = auth_service_secret[16..28].try_into().unwrap();
    let tx_nonce: [u8; 12] = auth_service_secret[28..40].try_into().unwrap();

    let encryption_key = CryptoKey::new(
        Timestamp {
            tv_sec: 0,
            tv_nsec: 0,
        },
        encryption_key,
    );

    connection.set_session_secrets(encryption_key, rx_nonce, tx_nonce);

    // let signature = connection.recv_done(&rx_auth);
    // send(signature, &mut stream);

    println!("Recv signature");

    let Message::AuthSignature(rx_sig) = recv(&mut connection, &mut stream) else {
        panic!("Expected AuthSignature, got something else");
    };

    println!("Signature rx: {rx_sig:?}");

    let rx_buf = connection.state().rx_buf.clone();
    let tx_buf = connection.state().tx_buf.clone();

    let tx_signature = auth_service_ticket.session_key.hmac_sha256(&rx_buf);
    let tx_signature = AuthSignature {
        sha256: tx_signature,
    };

    let _send = tx_signature;

    let mut connection = connection
        .recv_signature(Some(&auth_service_ticket.session_key), &tx_buf, &rx_sig)
        .unwrap();

    println!("Received signature was correct.");

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
}
