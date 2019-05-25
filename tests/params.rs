use tower_web::{derive_resource_impl, Serialize, Deserialize, Extract, Response, impl_web};

#[macro_use]
mod support;
use crate::support::*;

#[derive(Clone, Debug)]
struct TestParams;

impl_web! {
    impl TestParams {
        #[get("/str/:foo")]
        #[content_type("plain")]
        fn one_str_param(&self, foo: String) -> Result<&'static str, ()> {
            assert_eq!(foo, "hello");
            Ok("one_str_param")
        }

        #[get("/x-hello")]
        #[content_type("plain")]
        fn one_str_header(&self, x_hello: String) -> Result<&'static str, ()> {
            assert_eq!(x_hello, "world");
            Ok("one_str_header")
        }

        #[get("/u32/:foo")]
        #[content_type("plain")]
        fn one_u32_param(&self, foo: u32) -> Result<&'static str, ()> {
            assert_eq!(foo, 123);
            Ok("one_u32_param")
        }

        #[post("/content_length")]
        #[content_type("plain")]
        fn one_u32_header(&self, content_length: u32) -> Result<&'static str, ()> {
            assert_eq!(content_length, 5);
            Ok("one_u32_header")
        }

        #[get("/option_hdr")]
        #[content_type("plain")]
        fn option_header(&self, user_agent: Option<String>) -> Result<&'static str, ()> {
            if let Some(user_agent) = user_agent {
                assert_eq!(user_agent, "testin");
                Ok("option_header - some")
            } else {
                Ok("option_header - none")
            }
        }
    }
}

// TODO:
// - header missing

#[test]
fn one_str_param() {
    let mut web = service(TestParams);

    let response = web.call_unwrap(get!("/str/hello"));
    assert_ok!(response);
    assert_body!(response, "one_str_param");
}

#[test]
fn one_str_header() {
    let mut web = service(TestParams);


    let response = web.call_unwrap(get!("/x-hello", "x-hello": "world"));
    assert_ok!(response);
    assert_body!(response, "one_str_header");
}

#[test]
fn one_u32_param() {
    let mut web = service(TestParams);

    let response = web.call_unwrap(get!("/u32/123"));
    assert_ok!(response);
    assert_body!(response, "one_u32_param");
}

#[test]
fn one_u32_header() {
    let mut web = service(TestParams);

    let response = web.call_unwrap(post!("/content_length", "hello", "content-length": "5"));
    assert_ok!(response);
    assert_body!(response, "one_u32_header");
}

#[test]
fn option_header() {
    let mut web = service(TestParams);

    let response = web.call_unwrap(get!("/option_hdr"));
    assert_ok!(response);
    assert_body!(response, "option_header - none");

    let response = web.call_unwrap(get!("/option_hdr", "user-agent": "testin"));

    assert_ok!(response);
    assert_body!(response, "option_header - some");
}
