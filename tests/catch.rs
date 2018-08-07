extern crate http;
extern crate tower_service;
#[macro_use]
extern crate tower_web;

#[macro_use]
mod support;
use support::*;

#[derive(Clone, Debug)]
struct TestCatch;

#[derive(Clone, Debug)]
struct TestDefaultCatch;

impl_web! {
    impl TestCatch {
        /// @get("/buggy")
        fn buggy(&self) -> Result<String, ()> {
            Err(())
        }

        /// @get("/not_buggy")
        fn not_buggy(&self) -> Result<String, ()> {
            Ok("not buggy".to_string())
        }

        /// @catch
        fn catch(&self) -> Result<&'static str, ()> {
            Ok("catch")
        }
    }

    impl TestDefaultCatch {
        /// @get("/buggy")
        fn buggy(&self) -> Result<String, ()> {
            Err(())
        }
    }
}

#[test]
fn catch_error() {
    let mut web = service(TestCatch);

    let response = web.call_unwrap(get!("/buggy"));
    assert_ok!(response);
    assert_body!(response, "catch");
}

#[test]
fn success() {
    let mut web = service(TestCatch);

    let response = web.call_unwrap(get!("/not_buggy"));
    assert_ok!(response);
    assert_body!(response, "not buggy");
}

#[test]
fn default_catch_internal() {
    let mut web = service(TestDefaultCatch);

    let response = web.call_unwrap(get!("/buggy"));
    assert_internal_error!(response);
    assert_body!(response, "internal server error");
}

#[test]
fn default_catch_not_found() {
    let mut web = service(TestDefaultCatch);

    let response = web.call_unwrap(get!("/missing"));
    assert_not_found!(response);
    assert_body!(response, "not found");
}

#[test]
fn custom_global_catch() {
    use tower_service::NewService;

    let mut web = ::tower_web::ServiceBuilder::new()
        .resource(TestDefaultCatch)
        .catch(|_: &http::Request<()>, error: ::tower_web::Error| {
            assert!(error.kind().is_not_found());

            let response = http::response::Builder::new()
                .status(404)
                .header("content-type", "text/plain")
                .body(::tower_web::response::MapErr::new("where you at?"))
                .unwrap();

            Ok(response)
        })
        .build_new_service()
        .new_service()
        .wait().unwrap();

    let response = web.call_unwrap(get!("/missing"));
    assert_not_found!(response);
    assert_body!(response, "where you at?");
}
