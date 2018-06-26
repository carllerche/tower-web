extern crate futures;
extern crate http;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tower_web;
extern crate tower_service;

use tower_service::Service;
use tower_web::*;
use tower_web::util::BufStream;

use futures::{Future};
use http::{request, StatusCode};

#[derive(Clone, Debug)]
struct Basic;

#[derive(Serialize)]
struct Foo {
    bar: &'static str,
}

impl_web! {
    impl Basic {
        /// Hello world endpoint
        ///
        /// @get("/")
        /// @content_type("plain")
        fn sync_get_str(&mut self) -> Result<&'static str, ()> {
            Ok("hello world")
        }

        /// Get some json
        ///
        /// @get("/get-json")
        /// @content_type("json")
        fn sync_get_json(&mut self) -> Result<Foo, ()> {
            Ok(Foo { bar: "baz" })
        }
    }
}

#[test]
fn sync_get_str() {
    let mut service = ServiceBuilder::new()
        .resource(Basic)
        .build();

    let request = request::Builder::new()
        .method("GET")
        .uri("/")
        .body("".to_string())
        .unwrap();

    let response = service.call(request).wait().unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["content-type"], "text/plain");

    let body: Vec<u8> = response.into_body().collect().wait().unwrap();
    assert_eq!(body, &b"hello world"[..]);
}

#[test]
fn sync_get_json() {
    let mut service = ServiceBuilder::new()
        .resource(Basic)
        .build();

    let request = request::Builder::new()
        .method("GET")
        .uri("/get-json")
        .body("".to_string())
        .unwrap();

    let response = service.call(request).wait().unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()["content-type"], "application/json");

    let body: Vec<u8> = response.into_body().collect().wait().unwrap();
    assert_eq!(body, &b"{\"bar\":\"baz\"}"[..]);
}
