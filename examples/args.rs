/// # Hello world service.
///
/// A simple service that demonstrates how to get started with `tower-web`.
///
/// # Overview
///
/// `tower-web` lets you define a web service using a "plain old Rust type"
/// (PORT). The idea is to decouple all HTTP concepts from the business logic.
/// This is achieved by using macros that inspect the handler functions on your
/// PORT and generate a [Tower `Service`][service] implementation.
///
/// By doing this, `tower-web` also removes most of the boiler plate from
/// implementing web apps.
///
/// [service]: https://github.com/tower-rs/tower
///
/// ## Usage
///
/// Run the example:
///
///     cargo run --example hello_world
///
/// Then send a request:
///
///     curl -v http://localhost:8080/

#[macro_use]
extern crate tower_web;
extern crate tokio;

use tower_web::ServiceBuilder;
use tokio::prelude::*;

/// This type will be the web service implementation.
#[derive(Clone, Debug)]
pub struct HelloWorld {
    /// The message that will be included in the response to `/motd`.
    motd: String,
}

impl_web! {
    impl HelloWorld {
        /// @get("/")
        /// @content_type("plain")
        fn hello_world(&self) -> Result<&'static str, ()> {
            Ok("This is a basic response served by tower-web")
        }

        /// @get("/motd")
        /// @content_type("plain")
        fn motd(&self) -> Result<String, ()> {
            // You can also respond with an owned `String`.
            Ok(format!("MOTD: {}", self.motd))
        }

        /// @get("/hello-future")
        /// @content_type("plain")
        fn hello_future(&self) -> impl Future<Item = String, Error = ()> + Send {
            future::ok("Or return a future that resolves to the response".to_string())
        }

        /// @get("/hello-query-string")
        /// @content_type("plain")
        fn hello_query_string(&self, query_string: Option<MyArg>) -> Result<String, ()> {
            println!("QUERY: {:?}", query_string);
            Ok(format!("We received the query {:?}", query_string))
        }

        /// @get("/hello-query-string-required")
        /// @content_type("plain")
        fn hello_query_string_required(&self, query_string: MyArg) -> Result<String, ()> {
            println!("QUERY: {:?}", query_string);
            Ok(format!("We received the query {:?}", query_string))
        }

        /// @post("/users")
        /// @content_type("plain")
        fn create_user(&self, body: User) -> Result<String, ()> {
            println!("GOT = {:?}", body);
            Ok("We have received the user".to_string())
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HelloWorld)
        .run(&addr)
        .unwrap();
}
