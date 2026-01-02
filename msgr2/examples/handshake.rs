use std::{
    io::{Read, Write},
    net::TcpStream,
};

use ceph_protocol::{
    CephFeatureSet, CryptoKey, Encode, EntityAddress, EntityAddressType, EntityName, EntityType,
    Timestamp,
    connection::{Config, Connection, Message, states::Established},
    frame::Frame,
    messages::{
        Banner, ClientIdent, Hello, Keepalive,
        auth::{
            AuthMethodCephX, AuthMethodNone, AuthRequest, AuthRequestMore, CephXServerChallenge,
            ConMode,
        },
        cephx::{
            AuthServiceTicketInfo, CephXAuthenticate, CephXAuthenticateKey, CephXMessage,
            CephXMessageType, CephXTicketBlob,
        },
    },
};

fn send(frame: Frame<'_>, w: &mut impl std::io::Write) {
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

fn recv<S>(connection: &mut Connection<S>, r: &mut impl std::io::Read) -> Message
where
    S: Established,
{
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
    let master_key = CryptoKey::decode(include_bytes!("./key.bin")).unwrap();
    let mut stream = TcpStream::connect("10.0.1.222:3300").unwrap();

    let config = Config::new(true);
    let connection = ceph_protocol::connection::Connection::new(config);

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

    let challenge = CephXServerChallenge::parse(&more.payload).unwrap();

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
        other_keys: u8::from(EntityType::Mon) as u32,
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

    let auth_done = CephXMessage::parse(&rx_auth.auth_payload).unwrap();
    let tickets = auth_done.payload();

    match auth_done.ty() {
        CephXMessageType::GetAuthSessionKey => {
            let cbl = AuthServiceTicketInfo::parse(&tickets);

            // We are encrypted here: let's deal with that

            // panic!("{cbl:?}");
        }
        _ => unreachable!(),
    }

    let signature = connection.recv_done(&rx_auth);
    send(signature, &mut stream);

    let Message::AuthSignature(rx_sig) = recv(&mut connection, &mut stream) else {
        panic!("Expected AuthSignature, got something else");
    };

    println!("Signature rx: {rx_sig:?}");
    let mut connection = connection.recv_signature(&rx_sig).unwrap();

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
