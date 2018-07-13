extern crate futures;
extern crate http;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tower_web;

#[macro_use]
mod support;
use support::*;

#[derive(Clone, Debug)]
struct TestContentTypes;

#[derive(Serialize)]
struct Foo {
    bar: &'static str,
}

impl_web! {
    impl TestContentTypes {
        /// @get("/get-json")
        /// @content_type("json")
        fn sync_get_json(&mut self) -> Result<Foo, ()> {
            Ok(Foo { bar: "baz" })
        }
    }
}

#[test]
fn sync_get_json() {
    let mut web = service(TestContentTypes);

    let response = web.call_unwrap(get!("/get-json"));

    assert_ok!(response);
    assert_eq!(response.headers()["content-type"], "application/json");
    assert_body!(response, "{\"bar\":\"baz\"}");
}
