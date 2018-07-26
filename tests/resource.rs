extern crate futures;
extern crate http;
#[macro_use]
extern crate tower_web;

#[macro_use]
mod support;
use support::*;

#[derive(Clone, Debug)]
struct TestResource;

impl_web! {
    impl TestResource {
        /// @get("/impl_future")
        /// @content_type("plain")
        fn impl_future(&self) -> impl Future<Item = &'static str, Error = ()> {
            use futures::IntoFuture;
            Ok("impl_future").into_future()
        }
    }
}

#[test]
fn impl_future() {
    let mut web = service(TestResource);

    let response = web.call_unwrap(get!("/impl_future"));
    assert_ok!(response);
    assert_body!(response, "impl_future");
}
