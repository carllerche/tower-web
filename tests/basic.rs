extern crate futures;
extern crate http;
#[macro_use]
extern crate tower_web;
extern crate tower_service;

use tower_service::Service;
use tower_web::*;

use futures::{Future, Stream};
use http::{request, StatusCode};

#[derive(Clone, Debug)]
struct Basic;

impl_web! {
    impl Basic {
        /// Hello world endpoint
        ///
        /// @GET("/")
        fn sync_get_str(&mut self) -> Result<&'static str, ()> {
            Ok("hello world")
        }
    }
}

#[test]
fn sync_get_str() {
    let mut service = ServiceBuilder::new().resource(Basic).build();

    let request = request::Builder::new()
        .method("GET")
        .body("".to_string())
        .unwrap();

    let response = service.call(request).wait().unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().concat2().wait().unwrap();

    assert_eq!(body, b"hello world"[..]);
}
