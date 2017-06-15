use base64;
use crypto::digest::Digest;
use crypto::mac::Mac;
use crypto;
use diffie_hellman::{DH_GENERATOR, DH_PRIME};
use futures::sync::mpsc;
use futures::{Future, Stream, BoxFuture, Poll, Async};
use hyper::server::{Service, NewService, Request, Response, Http};
use hyper::{self, Get, Post, StatusCode};
use num_bigint::BigUint;
use rand;
use std::collections::BTreeMap;
use std::io;
use std::sync::Arc;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Handle;
use url;

use authentication::Credentials;
use util;

#[cfg(feature = "with-rust-mdns")]
use mdns;

#[cfg(feature = "with-avahi")]
use dns_sd::DNSService;

#[derive(Clone)]
struct Discovery(Arc<DiscoveryInner>);
struct DiscoveryInner {
    private_key: BigUint,
    public_key: BigUint,
    device_id: String,
    device_name: String,
    tx: mpsc::UnboundedSender<Credentials>,
}

impl Discovery {
    pub fn new(device_name: String, device_id: String)
        -> (Discovery, mpsc::UnboundedReceiver<Credentials>)
    {
        let (tx, rx) = mpsc::unbounded();

        let key_data = util::rand_vec(&mut rand::thread_rng(), 95);
        let private_key = BigUint::from_bytes_be(&key_data);
        let public_key = util::powm(&DH_GENERATOR, &private_key, &DH_PRIME);

        let discovery = Discovery(Arc::new(DiscoveryInner {
            device_name: device_name.to_owned(),
            device_id: device_id.to_owned(),
            private_key: private_key,
            public_key: public_key,
            tx: tx,
        }));

        (discovery, rx)
    }
}

impl Discovery {
    fn handle_get_info(&self, _params: &BTreeMap<String, String>)
        -> ::futures::Finished<Response, hyper::Error>
    {
        let public_key = self.0.public_key.to_bytes_be();
        let public_key = base64::encode(&public_key);

        let result = json!({
            "status": 101,
            "statusString": "ERROR-OK",
            "spotifyError": 0,
            "version": "2.1.0",
            "deviceID": (self.0.device_id),
            "remoteName": (self.0.device_name),
            "activeUser": "",
            "publicKey": (public_key),
            "deviceType": "UNKNOWN",
            "libraryVersion": "0.1.0",
            "accountReq": "PREMIUM",
            "brandDisplayName": "librespot",
            "modelDisplayName": "librespot",
        });

        let body = result.to_string();
        ::futures::finished(Response::new().with_body(body))
    }

    fn handle_add_user(&self, params: &BTreeMap<String, String>)
        -> ::futures::Finished<Response, hyper::Error>
    {
        let username = params.get("userName").unwrap();
        let encrypted_blob = params.get("blob").unwrap();
        let client_key = params.get("clientKey").unwrap();

        let encrypted_blob = base64::decode(encrypted_blob).unwrap();

        let client_key = base64::decode(client_key).unwrap();
        let client_key = BigUint::from_bytes_be(&client_key);

        let shared_key = util::powm(&client_key, &self.0.private_key, &DH_PRIME);

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
            h.input(b"checksum");
            h.result().code().to_owned()
        };

        let encryption_key = {
            let mut h = crypto::hmac::Hmac::new(crypto::sha1::Sha1::new(), &base_key);
            h.input(b"encryption");
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
                                              &encryption_key[0..16], iv);
            cipher.process(encrypted, &mut data);
            String::from_utf8(data).unwrap()
        };

        let credentials = Credentials::with_blob(username.to_owned(), &decrypted, &self.0.device_id);

        self.0.tx.send(credentials).unwrap();

        let result = json!({
            "status": 101,
            "spotifyError": 0,
            "statusString": "ERROR-OK"
        });

        let body = result.to_string();
        ::futures::finished(Response::new().with_body(body))
    }

    fn not_found(&self)
        -> ::futures::Finished<Response, hyper::Error>
    {
        ::futures::finished(Response::new().with_status(StatusCode::NotFound))
    }
}

impl Service for Discovery {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = BoxFuture<Response, hyper::Error>;

    fn call(&self, request: Request) -> Self::Future {
        let mut params = BTreeMap::new();

        let (method, uri, _, _, body) = request.deconstruct();
        if let Some(query) = uri.query() {
            params.extend(url::form_urlencoded::parse(query.as_bytes()).into_owned());
        }

        if method != Get {
            debug!("{:?} {:?} {:?}", method, uri.path(), params);
        }

        let this = self.clone();
        body.fold(Vec::new(), |mut acc, chunk| {
            acc.extend_from_slice(chunk.as_ref());
            Ok::<_, hyper::Error>(acc)
        }).map(move |body| {
            params.extend(url::form_urlencoded::parse(&body).into_owned());
            params
        }).and_then(move |params| {
            match (method, params.get("action").map(AsRef::as_ref)) {
                (Get, Some("getInfo")) => this.handle_get_info(&params),
                (Post, Some("addUser")) => this.handle_add_user(&params),
                _ => this.not_found(),
            }
        }).boxed()
    }
}

impl NewService for Discovery {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = Self;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

pub struct DiscoveryStream {
    credentials: mpsc::UnboundedReceiver<Credentials>,
    #[cfg(feature = "with-rust-mdns")]
    _svc: mdns::Service,
    #[cfg(feature = "with-avahi")]
    _svc: DNSService,
    task: Box<Future<Item=(), Error=io::Error>>,
}

pub fn discovery(handle: &Handle, device_name: String, device_id: String)
    -> io::Result<DiscoveryStream>
{
    let (discovery, creds_rx) = Discovery::new(device_name.clone(), device_id);

    let listener = TcpListener::bind(&"0.0.0.0:0".parse().unwrap(), handle)?;

    #[cfg(feature = "with-rust-mdns")]
    let addr = listener.local_addr()?;

    #[cfg(feature = "with-avahi")]
    let port = listener.local_addr().unwrap().port();

    let http = Http::new();
    let handle_ = handle.clone();
    let task = Box::new(listener.incoming().for_each(move |(socket, addr)| {
        http.bind_connection(&handle_, socket, addr, discovery.clone());
        Ok(())
    }));

    #[cfg(feature = "with-rust-mdns")]
    let responder = mdns::Responder::spawn(&handle)?;
    
    #[cfg(feature = "with-rust-mdns")]
    let svc = responder.register(
        "_spotify-connect._tcp".to_owned(),
        device_name,
        addr.port(),
        &["VERSION=1.0", "CPath=/"]);

    #[cfg(feature = "with-avahi")]
    let svc = DNSService::register(Some(&*device_name),
                                   "_spotify-connect._tcp",
                                   None,
                                   None,
                                   port,
                                   &["VERSION=1.0", "CPath=/"])
                      .unwrap();

    Ok(DiscoveryStream {
        credentials: creds_rx,
        _svc: svc,
        task: task,
    })
}

impl Stream for DiscoveryStream {
    type Item = Credentials;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.task.poll()? {
            Async::Ready(()) => unreachable!(),
            Async::NotReady => (),
        }

        Ok(self.credentials.poll().unwrap())
    }
}
