/// Add middleware to service
///
/// ## Usage
///
/// Run the example:
///
///     RUST_LOG="hello_world=info" cargo run --example hello_world
///
/// Then send a request:
///
///     curl -v http://localhost:8080/

extern crate env_logger;
#[macro_use]
extern crate tower_web;
extern crate tokio;

use tower_web::ServiceBuilder;
use tower_web::middleware::log::LogMiddleware;

#[derive(Clone, Debug)]
pub struct HelloWorld;

impl_web! {
    impl HelloWorld {
        /// @get("/")
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
        // We run the service
        .run(&addr)
        .unwrap();
}
