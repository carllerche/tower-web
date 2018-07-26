extern crate futures;
extern crate http;
#[macro_use]
extern crate tower_web;

#[macro_use]
mod support;
use support::*;

#[derive(Clone, Debug)]
struct TestExtract;

#[derive(Debug, Extract)]
pub struct Foo {
    foo: String,
}

#[derive(Debug, Extract)]
pub struct Foo2 {
    foo: Option<String>,
}

impl_web! {
    impl TestExtract {
        /// @get("/extract_query")
        /// @content_type("plain")
        fn extract_query(&self, query_string: Foo) -> Result<&'static str, ()> {
            assert_eq!(query_string.foo, "bar");
            Ok("extract_query")
        }

        /// @get("/extract_query_missing_ok")
        /// @content_type("plain")
        fn extract_query_missing_ok(&self, query_string: Foo2) -> Result<&'static str, ()> {
            if let Some(ref foo) = query_string.foo {
                assert_eq!(foo, "bar");
                Ok("extract_query_missing_ok - Some")
            } else {
                Ok("extract_query_missing_ok - None")
            }
        }

        /// @post("/extract_body")
        /// @content_type("plain")
        fn extract_body(&self, body: Foo) -> Result<&'static str, ()> {
            assert_eq!(body.foo, "body bar");
            Ok("extract_body")
        }
    }
}

#[test]
fn extract_query_success() {
    let mut web = service(TestExtract);

    let response = web.call_unwrap(get!("/extract_query?foo=bar"));
    assert_ok!(response);
    assert_body!(response, "extract_query");
}

#[test]
#[ignore]
fn extract_query_missing_not_ok() {
    let mut web = service(TestExtract);

    let response = web.call_unwrap(get!("/extract_query"));
    assert_bad_request!(response);
}

#[test]
fn extract_query_missing_ok() {
    let mut web = service(TestExtract);

    let response = web.call_unwrap(get!("/extract_query_missing_ok"));
    assert_ok!(response);
    assert_body!(response, "extract_query_missing_ok - None");
}

#[test]
fn extract_body_json_success() {
    let mut web = service(TestExtract);

    let body = r#"{"foo":"body bar"}"#;

    let response = web.call_unwrap(post!("/extract_body", body, "content-type": "application/json"));
    assert_ok!(response);
    assert_body!(response, "extract_body");
}
