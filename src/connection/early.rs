use byteorder::{BigEndian, ByteOrder};
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use futures::{Future, IntoFuture};
use protocol;
use protobuf::{self, Message, MessageStatic};
use rand::thread_rng;
use tokio::net::TcpStream;
use tokio::io::{write_all, read_exact, flush};

use diffie_hellman::DHLocalKeys;
use util;
use types::*;

pub struct EarlyConnection {
    stream: TcpStream,
    accumulator: Vec<u8>,
}

impl EarlyConnection {
    pub fn new(stream: TcpStream) -> EarlyConnection {
        EarlyConnection {
            stream: stream,
            accumulator: Vec::new(),
        }
    }

    fn send_packet_prefix<'a, M: Message>(self, prefix: &[u8], msg: M) -> SpFuture<'a, Self> {
        let EarlyConnection { stream, mut accumulator } = self;

        let size = prefix.len() + 4 + msg.compute_size() as usize;
        let mut size_field = [0u8; 4];
        BigEndian::write_u32(&mut size_field, size as u32);

        let mut buffer = Vec::with_capacity(size);
        buffer.extend_from_slice(prefix);
        buffer.extend_from_slice(&size_field);

        msg.write_to_vec(&mut buffer)
            .map_err(SpError::from)
            .into_future()
            .and_then(move |_| {
                accumulator.extend_from_slice(&buffer);

                write_all(stream, buffer)
                    .map_err(SpError::from)
                    .and_then(|(stream, _)| flush(stream).map_err(SpError::from))
                    .map(move |stream| {
                        EarlyConnection {
                            stream: stream,
                            accumulator: accumulator,
                        }
                    })
            })
            .sp_boxed()
    }

    fn send_packet<'a, M: Message>(self, msg: M) -> SpFuture<'a, Self> {
        self.send_packet_prefix(&[], msg)
    }

    fn recv_packet<'a, M: MessageStatic + Send>(self) -> SpFuture<'a, (Self, M)> {
        let EarlyConnection { stream, mut accumulator } = self;

        read_exact(stream, [0u8; 4])
            .map_err(SpError::from)
            .and_then(move |(stream, size)| {
                accumulator.extend_from_slice(&size);
                let size = BigEndian::read_u32(&size);
                read_exact(stream, vec![0u8; size as usize - 4])
                    .map_err(SpError::from)
                    .and_then(move |(stream, buffer)| {
                        accumulator.extend_from_slice(&buffer);
                        let message = try!(protobuf::parse_from_bytes(&buffer));
                        Ok((EarlyConnection {
                            stream: stream,
                            accumulator: accumulator,
                        },
                            message))
                    })
            })
            .sp_boxed()
    }
}

fn client_hello<'a>(connection: EarlyConnection, gc: Vec<u8>) -> SpFuture<'a, EarlyConnection> {
    let packet = protobuf_init!(protocol::keyexchange::ClientHello::new(), {
        build_info => {
            product: protocol::keyexchange::Product::PRODUCT_PARTNER,
            platform: protocol::keyexchange::Platform::PLATFORM_LINUX_X86,
            version: 0x10800000000,
        },
        cryptosuites_supported => [
            protocol::keyexchange::Cryptosuite::CRYPTO_SUITE_SHANNON,
        ],
        login_crypto_hello.diffie_hellman => {
            gc: gc,
            server_keys_known: 1,
        },
        client_nonce: util::rand_vec(&mut thread_rng(), 0x10),
        padding: vec![0x1e],
    });

    connection.send_packet_prefix(&[0, 4], packet)
}

fn ap_response<'a>(connection: EarlyConnection) -> SpFuture<'a, (EarlyConnection, Vec<u8>)> {
    connection.recv_packet::<protocol::keyexchange::APResponseMessage>()
        .map(|(connection, message)| {
            let remote_key = message.get_challenge()
                .get_login_crypto_challenge()
                .get_diffie_hellman()
                .get_gs()
                .to_owned();

            (connection, remote_key)
        })
        .sp_boxed()
}

fn client_response<'a>(connection: EarlyConnection,
                       challenge: Vec<u8>)
                       -> SpFuture<'a, EarlyConnection> {
    let packet = protobuf_init!(protocol::keyexchange::ClientResponsePlaintext::new(), {
        login_crypto_response.diffie_hellman => {
            hmac: challenge
        },
        pow_response => {},
        crypto_response => {},
    });

    connection.send_packet(packet)
}

pub fn keyexchange<'a>(connection: EarlyConnection) -> SpFuture<'a, (TcpStream, Vec<u8>, Vec<u8>)> {
    let local_keys = DHLocalKeys::random(&mut thread_rng());

    client_hello(connection, local_keys.public_key())
        .and_then(|connection| ap_response(connection))
        .and_then(move |(connection, remote_key)| {
            let shared_secret = local_keys.shared_secret(&remote_key);
            let (challenge, send_key, recv_key) = compute_keys(&shared_secret,
                                                               &connection.accumulator);

            client_response(connection, challenge)
                .map(move |connection| (connection.stream, send_key, recv_key))
        })
        .sp_boxed()
}

pub fn compute_keys(shared_secret: &[u8], packets: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>) {

    let mut data = Vec::with_capacity(0x64);
    let mut mac = Hmac::new(Sha1::new(), &shared_secret);

    for i in 1..6 {
        mac.input(packets);
        mac.input(&[i]);
        data.extend_from_slice(&mac.result().code());
        mac.reset();
    }

    mac = Hmac::new(Sha1::new(), &data[..0x14]);
    mac.input(packets);

    (mac.result().code().to_vec(), data[0x14..0x34].to_vec(), data[0x34..0x54].to_vec())
}
