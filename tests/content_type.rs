extern crate futures;
extern crate http;
#[macro_use]
extern crate tower_web;

#[macro_use]
mod support;
use support::*;

#[derive(Clone, Debug)]
struct TestContentType;

#[derive(Response)]
struct Foo {
    bar: &'static str,
}

impl_web! {
    impl TestContentType {
        /// @get("/str_no_content_type")
        fn str_no_content_type(&self) -> Result<&'static str, ()> {
            Ok("str_no_content_type")
        }

        /// @get("/str_with_content_type")
        /// @content_type("foo/bar")
        fn str_with_content_type(&self) -> Result<&'static str, ()> {
            Ok("str_with_content_type")
        }

        /// @get("/json_with_content_type")
        /// @content_type("json")
        fn json_with_content_type(&self) -> Result<Foo, ()> {
            Ok(Foo { bar: "baz" })
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
fn str_with_content_type() {
    let mut web = service(TestContentType);

    let response = web.call_unwrap(get!("/str_with_content_type"));
    assert_ok!(response);
    assert_header!(response, "content-type", "foo/bar");
    assert_body!(response, "str_with_content_type");
}

#[test]
fn json_with_content_type() {
    let mut web = service(TestContentType);

    let response = web.call_unwrap(get!("/json_with_content_type"));
    assert_ok!(response);
    assert_header!(response, "content-type", "application/json");
    assert_body!(response, "{\"bar\":\"baz\"}");
}
