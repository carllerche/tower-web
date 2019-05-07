//! Service with async/await handlers.
//!
//! A service that demonstrates how to use Tower Web's experimental support for the upcoming
//! async/await syntax.
//!
//! # Overview
//!
//! async/await enables easier writing of asynchronous code. Handler functions are prefaced with
//! the `async` keyword. The async fn implementation may then use `await!` and call other `async`
//! functions. It requires using the Rust nightly channel as well as opting into unstable features.
//!
//! ## Usage
//!
//! From within the `examples/async-await` directory, run the example:
//!
//!     cargo +nightly run --bin hyper
//!
//! Then send a request:
//!
//!     curl -v http://localhost:8080/

#![feature(await_macro, async_await)]

#[macro_use]
extern crate tower_web;
extern crate tokio;
extern crate hyper;

use tokio::prelude::*;
use tokio::await;
use tower_web::ServiceBuilder;

use std::str;

// The HTTP client
type HttpClient = hyper::Client<hyper::client::HttpConnector>;

/// This type will be part of the web service as a resource.
#[derive(Clone, Debug)]
pub struct HelloWorld {
    client: HttpClient,
}

/// To derive `Resource`, the implementation of `HelloWorld` is contained in the
/// `impl_web!` macro. This macro does not modify any of its contents. It will
/// inspect the implementation and, with the help of some annotations, generate
/// the necessary glue code to run the web service.
///
/// impl_web! is a temporary solution to enable tower-web to work with stable
/// rust. In the near future, this will be transitioned to use attribute macros.
impl_web! {
    impl HelloWorld {
        #[get("/")]
        async fn hello_world(&self) -> String {
            // Get the URI of the server by issuing a query to `/ip`.
            let uri = "http://httpbin.org/ip".parse().unwrap();

            // Issue the request and wait for the response
            let response = await!(self.client.get(uri)).unwrap();

            // Get the body component of the HTTP response. This is a stream and as such, it must
            // be asynchronously collected.
            let mut body = response.into_body();

            // The body chunks will be appended to this string.
            let mut ret = String::new();

            while let Some(chunk) = await!(body.next()) {
                let chunk = chunk.unwrap();

                // Convert to a string
                let chunk = str::from_utf8(&chunk[..]).unwrap();

                // Append to buffer
                ret.push_str(chunk);
            }

            // Return the collected string
            ret
        }
    }
}

pub fn main() {
    // Next, we must run our web service.
    //
    // The HTTP service will listen on this address and port.
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    // A service builder is used to configure our service.
    ServiceBuilder::new()
        .resource(HelloWorld {
            client: hyper::Client::new(),
        })
        .run(&addr)
        .unwrap();
}
