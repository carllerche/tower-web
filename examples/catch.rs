extern crate futures;
#[macro_use]
extern crate tower_web;
extern crate tokio;

use futures::{Future, IntoFuture};
use tower_web::ServiceBuilder;

use std::io;

#[derive(Clone, Debug)]
struct Buggy;

impl_web! {
    impl Buggy {
        #[get("/")]
        fn index(&self) -> impl Future<Item = String, Error = ()> {
            Err(()).into_future()
        }

        #[catch]
        fn catch_error(&self) -> impl Future<Item = String, Error = io::Error> {
            Ok("hello".to_string()).into_future()
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(Buggy)
        .run(&addr)
        .unwrap();
}
