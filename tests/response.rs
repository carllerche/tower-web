#![recursion_limit="256"]

extern crate futures;
extern crate http;
extern crate serde;
#[macro_use]
extern crate tower_web;

use serde::Serialize;

#[macro_use]
mod support;
use support::*;

#[derive(Debug)]
struct TestResponse;

#[derive(Debug)]
struct GenericTestResponse<T> {
    content: T,
}

#[derive(Debug, Response)]
pub struct HelloResponse {
    msg: &'static str,
}

#[derive(Debug, Response)]
pub struct NestedResponse {
    inner: Inner,
}

#[derive(Debug, Response)]
pub struct WrappedNestedResponse(Inner);

#[derive(Debug, Serialize)]
struct Inner {
    msg: &'static str,
}

#[derive(Debug, Response)]
#[web(status = "201")]
pub struct StaticStatus {
    msg: &'static str,
}

#[derive(Debug, Response)]
pub struct DynStatus {
    msg: &'static str,

    #[web(status)]
    status: u16,
}

#[derive(Debug, Response)]
#[web(header(name = "x-foo", value = "bar"))]
pub struct StaticHeader {
    msg: &'static str,
}

#[derive(Debug, Response)]
struct DynHeader1 {
    msg: &'static str,

    #[web(header)]
    x_bar: &'static str,
}

#[derive(Debug, Response)]
pub struct DynHeader2 {
    msg: &'static str,

    #[web(header(name = "x-baz"))]
    x_bar: &'static str,
}

#[derive(Debug, Response)]
#[web(either)]
pub enum Either {
    Something(String),
    SomethingElse(String),
}

#[derive(Response)]
struct GenResp<T> {
    inner: T,
}

// TODO: Fix the macro to avoid aliasing inner
type Msg = Inner;

impl_web! {
    impl TestResponse {
        #[get("/hello_world")]
        #[content_type("json")]
        fn hello_world(&self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse {
                msg: "hello world",
            })
        }

        #[get("/nested")]
        #[content_type("json")]
        fn nested(&self) -> Result<NestedResponse, ()> {
            Ok(NestedResponse {
                inner: Inner {
                    msg: "nested",
                }
            })
        }

        #[get("/wrapped_nested")]
        #[content_type("json")]
        fn wrapped_nested(&self) -> Result<WrappedNestedResponse, ()> {
            Ok(WrappedNestedResponse(Inner {
                msg: "nested",
            }))
        }

        #[get("/http_response")]
        #[content_type("plain")]
        fn http_response(&self) -> Result<http::Response<String>, ()> {
            http::Response::builder()
                .body("http_response".to_string())
                .map_err(|_| ())
        }

        #[get("/respond_static_status")]
        #[content_type("json")]
        fn respond_static_status(&self) -> Result<StaticStatus, ()> {
            Ok(StaticStatus {
                msg: "respond_static_status",
            })
        }

        #[get("/respond_dyn_status")]
        #[content_type("json")]
        fn respond_dyn_status(&self) -> Result<DynStatus, ()> {
            Ok(DynStatus {
                msg: "respond_dyn_status",
                status: 202,
            })
        }

        #[get("/respond_static_header")]
        #[content_type("json")]
        fn respond_static_header(&self) -> Result<StaticHeader, ()> {
            Ok(StaticHeader {
                msg: "respond_static_header",
            })
        }

        #[get("/respond_dyn_header_1")]
        #[content_type("json")]
        fn respond_dyn_header_1(&self) -> Result<DynHeader1, ()> {
            Ok(DynHeader1 {
                msg: "respond_dyn_header_1",
                x_bar: "baz",
            })
        }

        #[get("/respond_dyn_header_2")]
        #[content_type("json")]
        fn respond_dyn_header_2(&self) -> Result<DynHeader2, ()> {
            Ok(DynHeader2 {
                msg: "respond_dyn_header_2",
                x_bar: "not bar",
            })
        }

        #[get("/no_content_type")]
        fn no_content_type(&self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse {
                msg: "hello world",
            })
        }

        #[get("/gen_response")]
        #[content_type("json")]
        fn gen_response(&self) -> Result<GenResp<String>, ()> {
            Ok(GenResp {
                inner: "gen_response".to_string(),
            })
        }

        #[get("/option_response/:foo")]
        #[content_type("plain")]
        fn option_response(&self, foo: u32) -> Result<Option<String>, ()> {
            if foo == 1 {
                Ok(Some("hello".to_string()))
            } else {
                Ok(None)
            }
        }

        #[get("/vec_serialize")]
        #[content_type("json")]
        fn vec_serialize(&self) -> Result<Vec<Msg>, ()> {
            Ok(vec![Inner {
                msg: "vec_serialize",
            }])
        }
    }

    impl<T: Serialize + Clone> GenericTestResponse<T> {
        #[get("/generic")]
        fn generic(&self) -> Result<GenResp<T>, ()> {
            Ok(GenResp {
                inner: self.content.clone(),
            })
        }
    }
}

#[test]
fn hello_world() {
    let mut web = service(TestResponse);

    let response = web.call_unwrap(get!("/hello_world"));
    assert_ok!(response);
    assert_body!(response, "{\"msg\":\"hello world\"}");
}

#[test]
fn nested() {
    let mut web = service(TestResponse);

    let response = web.call_unwrap(get!("/nested"));
    assert_ok!(response);
    assert_body!(response, "{\"inner\":{\"msg\":\"nested\"}}");
}

#[test]
fn wrapped_nested() {
    let mut web = service(TestResponse);

    let response = web.call_unwrap(get!("/wrapped_nested"));
    assert_ok!(response);
    assert_body!(response, "{\"msg\":\"nested\"}");
}

#[test]
fn http_response() {
    let mut web = service(TestResponse);

    let response = web.call_unwrap(get!("/http_response"));
    assert_ok!(response);
    assert_body!(response, "http_response");
}

#[test]
fn respond_static_status() {
    let mut web = service(TestResponse);

    let response = web.call_unwrap(get!("/respond_static_status"));
    assert_created!(response);
    assert_body!(response, "{\"msg\":\"respond_static_status\"}");
}

#[test]
fn respond_dyn_status() {
    let mut web = service(TestResponse);

    let response = web.call_unwrap(get!("/respond_dyn_status"));
    assert_accepted!(response);
    assert_body!(response, "{\"msg\":\"respond_dyn_status\"}");
}

#[test]
fn respond_static_header() {
    let mut web = service(TestResponse);

    let response = web.call_unwrap(get!("/respond_static_header"));
    assert_ok!(response);
    assert_header!(response, "x-foo", "bar");
    assert_body!(response, "{\"msg\":\"respond_static_header\"}");
}

#[test]
fn respond_dyn_header_1() {
    let mut web = service(TestResponse);

    let response = web.call_unwrap(get!("/respond_dyn_header_1"));
    assert_ok!(response);
    assert_header!(response, "x-bar", "baz");
    assert_body!(response, "{\"msg\":\"respond_dyn_header_1\"}");
}

#[test]
fn respond_dyn_header_2() {
    let mut web = service(TestResponse);

    let response = web.call_unwrap(get!("/respond_dyn_header_2"));
    assert_ok!(response);
    assert_header!(response, "x-baz", "not bar");
    assert_body!(response, "{\"msg\":\"respond_dyn_header_2\"}");
}

#[test]
fn no_content_type() {
    let mut web = service(TestResponse);

    let response = web.call_unwrap(get!("/no_content_type"));
    assert_internal_error!(response);
}

#[test]
fn generic_response() {
    let mut web = service(TestResponse);

    let response = web.call_unwrap(get!("/gen_response"));
    assert_ok!(response);
    assert_body!(response, "{\"inner\":\"gen_response\"}");
}

#[test]
fn option_response() {
    let mut web = service(TestResponse);

    let response = web.call_unwrap(get!("/option_response/1"));
    assert_ok!(response);
    assert_body!(response, "hello");

    let response = web.call_unwrap(get!("/option_response/0"));
    assert_not_found!(response);
}

#[test]
fn vec_serialize() {
    let mut web = service(TestResponse);

    let response = web.call_unwrap(get!("/vec_serialize"));
    assert_ok!(response);
    assert_body!(response, r#"[{"msg":"vec_serialize"}]"#);
}
