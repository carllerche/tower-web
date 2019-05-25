#[macro_use]
extern crate serde_json;
use tower_web::{derive_resource_impl, Serialize, Deserialize, Extract, Response, impl_web};

#[macro_use]
mod support;
use crate::support::*;

use serde_json::Value;

#[derive(Clone, Debug)]
struct TestContentType;

#[derive(Response)]
struct Foo {
    bar: &'static str,
}

impl_web! {
    impl TestContentType {
        #[get("/str_no_content_type")]
        fn str_no_content_type(&self) -> Result<&'static str, ()> {
            Ok("str_no_content_type")
        }

        #[get("/str_no_content_type.html")]
        fn str_no_content_type_html(&self) -> Result<&'static str, ()> {
            Ok("<html></html>")
        }

        #[get("/str_with_content_type")]
        #[content_type("foo/bar")]
        fn str_with_content_type(&self) -> Result<&'static str, ()> {
            Ok("str_with_content_type")
        }

        #[get("/json_no_content_type")]
        fn json_no_content_type(&self) -> Result<Value, ()> {
            Ok(json!({
                "foo": "hello world",
            }))
        }

        #[get("/json_with_content_type")]
        #[content_type("json")]
        fn json_with_content_type(&self) -> Result<Foo, ()> {
            Ok(Foo { bar: "baz" })
        }

        #[get("/json_with_content_type2")]
        #[content_type("foo/bar")]
        fn json_with_content_type2(&self) -> Result<Value, ()> {
            Ok(json!({
                "foo": "hello world",
            }))
        }
    }
}

#[test]
fn str_no_content_type() {
    let mut web = service(TestContentType);

    let response = web.call_unwrap(get!("/str_no_content_type"));
    assert_ok!(response);
    assert_header!(response, "content-type", "text/plain");
    assert_body!(response, "str_no_content_type");
}

#[test]
fn str_no_content_type_html() {
    let mut web = service(TestContentType);

    let response = web.call_unwrap(get!("/str_no_content_type.html"));
    assert_ok!(response);
    assert_header!(response, "content-type", "text/html");
    assert_body!(response, "<html></html>");
}

#[test]
fn str_with_content_type() {
    let mut web = service(TestContentType);

    let response = web.call_unwrap(get!("/str_with_content_type"));
    assert_ok!(response);
    assert_header!(response, "content-type", "foo/bar");
    assert_body!(response, "str_with_content_type");
}

#[test]
fn json_no_content_type() {
    let mut web = service(TestContentType);

    let response = web.call_unwrap(get!("/json_no_content_type"));
    assert_ok!(response);
    assert_header!(response, "content-type", "application/json");
    assert_body!(response, "{\"foo\":\"hello world\"}");
}

#[test]
fn json_with_content_type() {
    let mut web = service(TestContentType);

    let response = web.call_unwrap(get!("/json_with_content_type"));
    assert_ok!(response);
    assert_header!(response, "content-type", "application/json");
    assert_body!(response, "{\"bar\":\"baz\"}");
}

#[test]
fn json_with_content_type2() {
    let mut web = service(TestContentType);

    let response = web.call_unwrap(get!("/json_with_content_type2"));
    assert_ok!(response);
    assert_header!(response, "content-type", "foo/bar");
    assert_body!(response, "{\"foo\":\"hello world\"}");
}
