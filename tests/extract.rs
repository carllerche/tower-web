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

#[derive(Debug, Extract, Default)]
struct FooWithDefault {
    #[serde(default)]
    foo: String,
}

#[derive(Debug, Extract)]
pub struct FooWrap(Inner);

#[derive(Debug, Deserialize)]
pub struct Inner {
    foo: String,
}

impl_web! {
    impl TestExtract {
        #[get("/extract_query")]
        #[content_type("plain")]
        fn extract_query(&self, query_string: Foo) -> Result<&'static str, ()> {
            assert_eq!(query_string.foo, "bar");
            Ok("extract_query")
        }

        #[get("/extract_query_wrap")]
        #[content_type("plain")]
        fn extract_query_wrap(&self, query_string: FooWrap) -> Result<&'static str, ()> {
            assert_eq!(query_string.0.foo, "bar");
            Ok("extract_query_wrap")
        }

        #[get("/extract_query_missing_ok")]
        #[content_type("plain")]
        fn extract_query_missing_ok(&self, query_string: Foo2) -> Result<&'static str, ()> {
            if let Some(ref foo) = query_string.foo {
                assert_eq!(foo, "bar");
                Ok("extract_query_missing_ok - Some")
            } else {
                Ok("extract_query_missing_ok - None")
            }
        }

        #[post("/extract_body")]
        #[content_type("plain")]
        fn extract_body(&self, body: Foo) -> Result<&'static str, ()> {
            assert_eq!(body.foo, "body bar");
            Ok("extract_body")
        }

        #[post("/extract_body_wrap")]
        #[content_type("plain")]
        fn extract_body_wrap(&self, body: FooWrap) -> Result<&'static str, ()> {
            assert_eq!(body.0.foo, "body bar");
            Ok("extract_body_wrap")
        }

        #[post("/extract_body_str")]
        #[content_type("plain")]
        fn extract_body_str(&self, body: String) -> Result<String, ()> {
            let mut out = "extract_body_str\n".to_string();
            out.push_str(&body);
            Ok(out)
        }

        #[post("/extract_x_www_form_urlencoded")]
        #[content_type("plain")]
        fn extract_x_www_form_urlencoded(&self, body: Foo) -> Result<&'static str, ()> {
            assert_eq!(body.foo, "body bar");
            Ok("extract_x_www_form_urlencoded")
        }

        #[get("/extract_with_default")]
        #[content_type("plain")]
        fn extract_with_default(&self, query_string: FooWithDefault) -> Result<&'static str, ()> {
            assert_eq!(query_string.foo, "");
            Ok("extract_with_default")
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
#[ignore]
fn extract_query_wrap() {
    let mut web = service(TestExtract);

    let response = web.call_unwrap(get!("/extract_query_wrap?foo=bar"));
    assert_ok!(response);
    assert_body!(response, "extract_query");
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

#[test]
fn extract_body_json_success_charset() {
    let mut web = service(TestExtract);

    let body = r#"{"foo":"body bar"}"#;

    let response = web.call_unwrap(post!("/extract_body", body, "content-type": "application/json;charset=utf-8"));
    assert_ok!(response);
    assert_body!(response, "extract_body");
}

#[test]
fn extract_body_wrap_json_success() {
    let mut web = service(TestExtract);

    let body = r#"{"foo":"body bar"}"#;

    let response = web.call_unwrap(post!("/extract_body_wrap", body, "content-type": "application/json"));
    assert_ok!(response);
    assert_body!(response, "extract_body_wrap");
}

#[test]
fn extract_body_wrap_json_no_content_type_header() {
    let mut web = service(TestExtract);

    let body = "";

    let response = web.call_unwrap(post!("/extract_body", body));
    assert_bad_request!(response);
}

#[test]
fn extract_x_www_form_urlencoded() {
    let mut web = service(TestExtract);

    let body = "foo=body bar";

    let response = web.call_unwrap(post!("/extract_x_www_form_urlencoded", body, "content-type": "application/x-www-form-urlencoded"));
    assert_ok!(response);
    assert_body!(response, "extract_x_www_form_urlencoded");
}

#[test]
fn extract_with_default() {
    let mut web = service(TestExtract);

    let response = web.call_unwrap(get!("/extract_with_default"));
    assert_ok!(response);
    assert_body!(response, "extract_with_default");
}

#[test]
fn extract_str() {
    let mut web = service(TestExtract);

    let body = "zomg a body";

    let response = web.call_unwrap(
        post!("/extract_body_str", body, "content-type": "text/plain"));

    assert_ok!(response);
    assert_body!(response, "extract_body_str\nzomg a body");

    // ensure the body is *not* decoded

    let mut web = service(TestExtract);

    let body = "zomg %20 body";

    let response = web.call_unwrap(
        post!("/extract_body_str", body, "content-type": "text/plain"));

    assert_ok!(response);
    assert_body!(response, "extract_body_str\nzomg %20 body");
}
