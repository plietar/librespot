use crypto;
use crypto::digest::Digest;
use crypto::mac::Mac;
use diffie_hellman::{DH_GENERATOR, DH_PRIME};
use futures::sync::mpsc;
use futures::{self, BoxFuture, Future, Stream, Poll};
use hyper;
use hyper::server::{Request, Response};
use mdns;
use num::BigUint;
use rand;
use rustc_serialize::base64::{self, ToBase64, FromBase64};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::reactor::Handle;
use tokio_service::{NewService, Service};
use url;

use authentication::Credentials;
use util;

// TODO: how do we stop the hyper server ?
pub struct Discovery {
    svc: mdns::Service,
    rx: mpsc::UnboundedReceiver<Credentials>,
}

impl Discovery {
    pub fn new(handle: &Handle, device_name: String, device_id: String) -> io::Result<Discovery> {
        let (tx, rx) = mpsc::unbounded();
        let key_data = util::rand_vec(&mut rand::thread_rng(), 95);
        let private_key = BigUint::from_bytes_be(&key_data);
        let public_key = util::powm(&DH_GENERATOR, &private_key, &DH_PRIME);

        let new_service = DiscoveryService {
            tx: RefCell::new(tx),
            data: Arc::new(DiscoveryData {
                private_key: private_key,
                public_key: public_key,
                device_id: device_id,
                device_name: device_name.clone(),
            }),
        };

        let addr = "0.0.0.0:0".to_socket_addrs()?.next().unwrap();
        let server = hyper::Server::http(&addr).unwrap();
        let bound_addr = server.handle(new_service, handle).unwrap();
        let port = bound_addr.port();

        let responder = mdns::Responder::new()?;
        let svc = responder.register(
            "_spotify-connect._tcp".to_owned(),
            device_name.to_owned(),
            port,
            &["VERSION=1.0", "CPath=/"]
        );

        Ok(Discovery {
            svc: svc,
            rx: rx,
        })
    }
}

impl Stream for Discovery {
    type Item = Credentials;
    type Error = io::Error;
    fn poll(&mut self) -> Poll<Option<Credentials>, io::Error> {
        Ok(self.rx.poll().unwrap())
    }
}

struct DiscoveryData {
    private_key: BigUint,
    public_key: BigUint,
    device_id: String,
    device_name: String,
}

#[derive(Clone)]
struct DiscoveryService {
    tx: RefCell<mpsc::UnboundedSender<Credentials>>,
    data: Arc<DiscoveryData>,
}

impl NewService for DiscoveryService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = DiscoveryService;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

fn parse_params<B>(query: Option<String>, body: Option<B>) -> BoxFuture<BTreeMap<String, String>, hyper::Error>
    where B: Stream<Item=Vec<u8>, Error=hyper::Error> + Send + 'static
{
    let mut params = query.map(|q| {
        url::form_urlencoded::parse(q.as_bytes())
            .into_owned()
            .collect()
    }).unwrap_or(BTreeMap::new());

    if let Some(body) = body {
        body.fold(Vec::new(), |mut a, b| -> Result<Vec<u8>, hyper::Error> { a.extend(b.iter()); Ok(a) })
            .map(move |body: Vec<u8>| {
                let form = url::form_urlencoded::parse(&body).into_owned();
                params.extend(form);
                params
            })
            .boxed()
    } else {
        futures::finished(params).boxed()
    }
}

impl Service for DiscoveryService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, request: Self::Request) -> Self::Future {
        if let hyper::uri::RequestUri::AbsolutePath { path, query } = request.uri().clone() {
            let method = request.method().clone();
            let body = if method == hyper::method::Method::Post {
                Some(request.body())
            } else {
                None
            };

            let svc = self.clone();
            parse_params(query, body.map(|body| body.map(|chunk| chunk.to_vec())))
                .map(move |params| {
                    debug!("{:?} {:?} {:?}", method, path, params);
                    let response = match params.get("action").map(AsRef::as_ref) {
                        Some("getInfo") => svc.handle_get_info(&params),
                        Some("addUser") => svc.handle_add_user(&params),
                        _ => svc.not_found(),
                    };
                    response
                }).boxed()
        } else {
            futures::finished(self.not_found()).boxed()
        }
    }
}

impl DiscoveryService {
    fn handle_get_info(&self, _params: &BTreeMap<String, String>) -> Response {
        let public_key =
            self.data.public_key
                .to_bytes_be()
                .to_base64(base64::STANDARD);

        let result = json!({
            "status": 101,
            "statusString": "ERROR-OK",
            "spotifyError": 0,
            "version": "2.1.0",
            "deviceID": (self.data.device_id),
            "remoteName": (self.data.device_name),
            "activeUser": "",
            "publicKey": (public_key),
            "deviceType": "UNKNOWN",
            "libraryVersion": "0.1.0",
            "accountReq": "PREMIUM",
            "brandDisplayName": "librespot",
            "modelDisplayName": "librespot",
        });

        Response::new().body(result.to_string().into_bytes())
    }

    fn handle_add_user(&self, params: &BTreeMap<String, String>) -> Response {
        let username = params.get("userName").unwrap();
        let encrypted_blob = params.get("blob").unwrap();
        let client_key = params.get("clientKey").unwrap();

        let encrypted_blob = encrypted_blob.from_base64().unwrap();

        let client_key = client_key.from_base64().unwrap();
        let client_key = BigUint::from_bytes_be(&client_key);

        let shared_key = util::powm(&client_key, &self.data.private_key, &DH_PRIME);

        let iv = &encrypted_blob[0..16];
        let encrypted = &encrypted_blob[16..encrypted_blob.len() - 20];
        let cksum = &encrypted_blob[encrypted_blob.len() - 20..encrypted_blob.len()];

        let base_key = {
            let mut data = [0u8; 20];
            let mut h = crypto::sha1::Sha1::new();
            h.input(&shared_key.to_bytes_be());
            h.result(&mut data);
            data[..16].to_owned()
        };

        let checksum_key = {
            let mut h = crypto::hmac::Hmac::new(crypto::sha1::Sha1::new(), &base_key);
            h.input("checksum".as_bytes());
            h.result().code().to_owned()
        };

        let encryption_key = {
            let mut h = crypto::hmac::Hmac::new(crypto::sha1::Sha1::new(), &base_key);
            h.input("encryption".as_bytes());
            h.result().code().to_owned()
        };

        let mac = {
            let mut h = crypto::hmac::Hmac::new(crypto::sha1::Sha1::new(), &checksum_key);
            h.input(encrypted);
            h.result().code().to_owned()
        };

        assert_eq!(&mac[..], cksum);

        let decrypted = {
            let mut data = vec![0u8; encrypted.len()];
            let mut cipher = crypto::aes::ctr(crypto::aes::KeySize::KeySize128,
                                              &encryption_key[0..16],
                                              &iv);
            cipher.process(&encrypted, &mut data);
            String::from_utf8(data).unwrap()
        };

        let credentials = Credentials::with_blob(username.to_owned(), &decrypted, &self.data.device_id);

        self.tx.borrow_mut().send(credentials).unwrap();

        let result = json!({
            "status": 101,
            "spotifyError": 0,
            "statusString": "ERROR-OK"
        });

        Response::new().body(result.to_string().into_bytes())
    }

    fn not_found(&self) -> Response {
        Response::new().status(hyper::status::StatusCode::NotFound)
    }
}
