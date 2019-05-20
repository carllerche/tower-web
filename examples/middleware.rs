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
#[macro_use]
extern crate tower_web;

use tower_web::ServiceBuilder;
use tower_web::middleware::deflate::DeflateMiddleware;
use tower_web::middleware::log::LogMiddleware;

use flate2::Compression;

#[derive(Clone, Debug)]
pub struct HelloWorld;

impl_web! {
    impl HelloWorld {
        #[get("/")]
        fn hello_world(&self) -> Result<&'static str, ()> {
            Ok("hello world")
        }
    }
}

pub fn main() {
    let _ = env_logger::try_init();

    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HelloWorld)
        // Add middleware, in this case access logging
        .middleware(LogMiddleware::new("hello_world::web"))
        .middleware(DeflateMiddleware::new(Compression::best()))
        // We run the service
        .run(&addr)
        .unwrap();
}
