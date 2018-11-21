extern crate tokio;
#[macro_use]
extern crate tower_web;
extern crate tokio_rustls;
use tokio::prelude::*;
use tokio::net::TcpListener;
use tower_web::ServiceBuilder;
use std::env;
use std::net::SocketAddr;
use std::io::{BufReader, Error};
use std::sync::Arc;
use tokio_rustls::{
    TlsAcceptor,
    rustls::{
        Certificate, NoClientAuth, PrivateKey, ServerConfig,
        internal::pemfile::{ certs, rsa_private_keys },
    },
};

fn load_certs(path: &str) -> Vec<Certificate> {
    certs(&mut BufReader::new(std::fs::File::open(path).unwrap())).unwrap()
}

fn load_private_keys(path: &str) -> Vec<PrivateKey> {
    rsa_private_keys(&mut BufReader::new(std::fs::File::open(path).unwrap())).unwrap()
}

pub struct TestResponse {
    msg: String,
}

impl_web! {
    impl TestResponse {
        #[get("/")]
        fn index(&self) -> Result<String, ()> {
            Ok(self.msg.clone())
        }
    }
}

pub fn main() {
    let addr: SocketAddr = match env::var("ADDRESS") {
        Ok(a) => a.parse().unwrap(),
        Err(_)  => "127.0.0.1:8443".parse().unwrap(),
    };

    // wrap_with_tls is a TlsAcceptor that can be used to wrap the TCP socket with TLS
    //
    // To generate self signed certs for testing on localhost you can use the
    // following command:
    //    openssl genrsa 4096 >local.key.pem && openssl req -new -x509 -key local.key.pem -out local.cert.pem -days 30 -subj '/CN=localhost/'
    //
    //  If using self signed certs you will receive the following error messages
    //  emanating from the browser until you accept the certificate in the browser
    //  and are allowed to procede to the site and view the Hello World index page:
    //    Error [TLS]: Custom { kind: InvalidData, error: AlertReceived(BadCertificate) }
    let tls_config = {
        let mut config = ServerConfig::new(NoClientAuth::new());
        config.set_single_cert(load_certs("local.cert.pem"), load_private_keys("local.key.pem").remove(0))
            .expect("invalid key or certificate");
        TlsAcceptor::from(Arc::new(config))
    };

    // There is probably a simpler way to set this up, however,
    // here are the steps I use to wrap TCP with TLS
    let incoming = TcpListener::bind(&addr).unwrap()
        .incoming()
        .map(move |tcp_stream| tls_config.accept(tcp_stream)) // Returns an Accept Future
        .and_then(|tls_stream| tls_stream) // Turn Accept Future into Result<Option<TlsStream>>
        .then(|r| match r {
            Ok(x) => Ok::<_, Error>(Some(x)),
            Err(e) => {
                // Catch and log TLS errors here
                eprintln!("Error [TLS]: {:?}", e);
                Ok(None)
            },
        })
        .filter_map(|x| x); // Skip over None(s) providing only unwrapped valid TlsStream(s)

    println!("Listening on https://{}", addr);
    tokio::run({
        ServiceBuilder::new()
        .resource(TestResponse{
            msg: "Hello World!".to_string(),
        })
        .serve(incoming)
    })
}
