/// Web service with a custom type argument.
///
/// ## Overview
///
/// Custom types can be used as arguments to web handler functions.
/// `[derive(Extract)]`.
///
/// ## Usage
///
/// Run the example:
///
///     cargo run --example derive_extract
///
/// Then send a request:
///
///     curl -v http://localhost:8080/query-string?foo=123

#[macro_use]
extern crate tower_web;
extern crate tokio;

use tower_web::ServiceBuilder;

/// This type will be the web service implementation.
#[derive(Clone, Debug)]
pub struct ArgResource;

#[derive(Debug, Extract)]
struct Foo {
    /// A `foo` component must be provided and it must be a numeric type.
    foo: u32,

    /// A `bar` component is always optional
    bar: Option<String>,
}

impl_web! {
    impl ArgResource {

        // By convention, arguments named `query_string` will be populated using
        // the HTTP request query string.
        #[get("/query-string")]
        fn hello_query_string(&self, query_string: Option<Foo>) -> Result<String, ()> {
            Ok(format!("We received the query {:?}", query_string))
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(ArgResource)
        .run(&addr)
        .unwrap();
}
