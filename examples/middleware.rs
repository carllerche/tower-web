/// Add middleware to service

#[macro_use]
extern crate tower_web;
extern crate tokio;

use tower_web::ServiceBuilder;
use tower_web::middleware::cors::CorsBuilder;
use tokio::prelude::*;

#[derive(Clone, Debug)]
pub struct HelloWorld;

impl_web! {
    impl HelloWorld {

        /// @get("/")
        /// @content_type("plain")
        fn hello_world(&self) -> Result<&'static str, ()> {
            Ok("hello world")
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HelloWorld)
        // Add middleware
        .middleware({
            CorsBuilder::permissive()
                .build()
        })
        // We run the service
        .run(&addr)
        .unwrap();
}
