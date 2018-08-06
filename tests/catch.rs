extern crate http;
#[macro_use]
extern crate tower_web;

#[macro_use]
mod support;
use support::*;

#[derive(Clone, Debug)]
struct TestCatch;

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
