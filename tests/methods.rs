#![recursion_limit = "128"]

extern crate futures;
extern crate http;
#[macro_use]
extern crate tower_web;

#[macro_use]
mod support;
use support::*;

use http::request;

#[derive(Clone, Debug)]
struct TestMethods;

impl_web! {
    impl TestMethods {
        #[get("/")]
        #[content_type("plain")]
        fn sync_get_str(&self) -> Result<&'static str, ()> {
            Ok("GET: hello world")
        }

        #[post("/")]
        #[content_type("plain")]
        fn sync_post_str(&self) -> Result<&'static str, ()> {
            Ok("POST: hello world")
        }

        #[put("/")]
        #[content_type("plain")]
        fn sync_put_str(&self) -> Result<&'static str, ()> {
            Ok("PUT: hello world")
        }

        #[patch("/")]
        #[content_type("plain")]
        fn sync_patch_str(&self) -> Result<&'static str, ()> {
            Ok("PATCH: hello world")
        }

        #[delete("/")]
        #[content_type("plain")]
        fn sync_delete_str(&self) -> Result<&'static str, ()> {
            Ok("DELETE: hello world")
        }
    }
}

#[test]
fn sync_method_str() {
    let mut web = service(TestMethods);

    let methods = [
        "GET",
        "POST",
        "PUT",
        "PATCH",
        "DELETE",
    ];

    for &method in &methods {
        let request = request::Builder::new()
            .method(method)
            .uri("/")
            .body("".to_string())
            .unwrap();

        let response = web.call_unwrap(request);
        assert_ok!(response);
        assert_eq!(response.headers()["content-type"], "text/plain");
        assert_body!(response, format!("{}: hello world", method));
    }
}
