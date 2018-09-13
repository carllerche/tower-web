/// Add middleware to service
///
/// ## Overview
///
/// Middleware decorates a service, adding additional functionality. It is a
/// concept common to most web frameworks.
///
/// Tower Web uses the Tower stack for middleware (hence the name). This example
/// decorates the application with the LogMiddleware. This middleware logs
/// information for each request.
///
/// ## Usage
///
/// Run the example:
///
///     RUST_LOG="hello_world=info" cargo run --example middleware
///
/// Then send a request:
///
///     curl -v http://localhost:8080/
extern crate env_logger;
extern crate flate2;
#[macro_use]
extern crate tower_web;
extern crate prometheus;
extern crate tokio;

use prometheus::{Encoder, Registry, TextEncoder};
use tower_web::middleware::deflate::DeflateMiddleware;
use tower_web::middleware::log::LogMiddleware;
use tower_web::middleware::prometheus::PrometheusMiddleware;
use tower_web::ServiceBuilder;

use flate2::Compression;

#[derive(Clone)]
pub struct HelloWorld {
    registry: Registry,
}

impl_web! {
    impl HelloWorld {
        #[get("/")]
        fn hello_world(&self) -> Result<&'static str, ()> {
            Ok("hello world")
        }

        #[get("/metrics")]
        fn metrics(&self) -> Result<String, ()> {
            let encoder = TextEncoder::new();
            let metric_families = self.registry.gather();
            let mut buf = vec![];
            encoder.encode(&metric_families, &mut buf).unwrap();
            Ok(std::string::String::from_utf8_lossy(&buf).to_string())
        }
    }
}

pub fn main() {
    let _ = env_logger::try_init();

    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    let registry = prometheus::Registry::new();

    ServiceBuilder::new()
        .resource(HelloWorld{ registry: registry.clone() })
        // Add middleware, in this case access logging
        .middleware(LogMiddleware::new("hello_world::web"))
        .middleware(DeflateMiddleware::new(Compression::best()))
        .middleware(PrometheusMiddleware::new("example", registry))
        // We run the service
        .run(&addr)
        .unwrap();
}
