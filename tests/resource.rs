#![recursion_limit="256"]

extern crate futures;
extern crate http;
#[macro_use]
extern crate tower_web;

use std::fmt::Display;

#[macro_use]
mod support;
use support::*;

#[derive(Clone, Debug)]
struct TestResource;

#[derive(Clone, Debug)]
struct HelloWorld<S>(S);

#[derive(Clone, Debug)]
struct WhereHelloWorld<S>(S);

#[derive(Clone, Debug)]
struct EmptyGeneric<S>(S);

#[derive(Clone, Debug)]
struct EmptyWhere<S>(S);

impl_web! {
    impl TestResource {
        #[get("/impl_future")]
        #[content_type("plain")]
        fn impl_future(&self) -> impl Future<Item = &'static str, Error = ()> {
            use futures::IntoFuture;
            Ok("impl_future").into_future()
        }
    }

    impl<S: Display> HelloWorld<S> {
        #[get("/")]
        fn hello(&self) -> Result<String, ()> {
            let r = self.0.to_string();
            Ok(r)
        }
    }

    impl<S> WhereHelloWorld<S>
    where
        S: Display,
    {
        #[get("/")]
        fn hello(&self) -> Result<String, ()> {
            let r = self.0.to_string();
            Ok(r)
        }
    }

    // Just make sure this compiles
    impl<S> EmptyGeneric<S> {
    }

    impl<S> EmptyWhere<S> where S: Display {
    }
}

#[test]
fn impl_future() {
    let mut web = service(TestResource);

    let response = web.call_unwrap(get!("/impl_future"));
    assert_ok!(response);
    assert_body!(response, "impl_future");
}

#[test]
fn generic() {
    let mut web = service(HelloWorld("hello"));

    let response = web.call_unwrap(get!("/"));
    assert_ok!(response);
    assert_body!(response, "hello");

    let mut web = service(WhereHelloWorld("hello"));

    let response = web.call_unwrap(get!("/"));
    assert_ok!(response);
    assert_body!(response, "hello");
}
