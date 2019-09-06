#![recursion_limit="256"]

use futures;
use tower_web::{derive_resource_impl, Serialize, Response, impl_web};

use std::fmt::Display;

mod support;
use crate::support::*;

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

#[derive(Response)]
struct Inner<T>(T);

#[derive(Serialize)]
struct GeneratedResource<T>(T);

#[derive(Serialize)]
struct ResponseFuture {
    msg: &'static str,
}

impl_web! {
    impl TestResource {
        #[get("/impl_future")]
        #[content_type("plain")]
        fn impl_future(&self) -> impl Future<Item = &'static str, Error = ()> {
            use futures::IntoFuture;
            Ok("impl_future").into_future()
        }

        #[get("/inner")]
        #[content_type("json")]
        fn inner(&self) -> Result<Inner<GeneratedResource<ResponseFuture>>, ()> {
            Ok(Inner(GeneratedResource(ResponseFuture {
                msg: "hello",
            })))
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

#[test]
fn inner_type() {
    let mut web = service(TestResource);

    let response = web.call_unwrap(get!("/inner"));
    assert_ok!(response);
    assert_body!(response, r#"{"msg":"hello"}"#);
}
