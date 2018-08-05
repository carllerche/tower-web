extern crate futures;
#[macro_use]
extern crate tower_web;
extern crate tokio;

use futures::{Future, IntoFuture};
use tower_web::ServiceBuilder;

use std::io;

#[derive(Clone, Debug)]
struct Buggy;

struct Catch;

impl_web! {
    impl Buggy {
        /// @get("/")
        fn index(&self) -> impl Future<Item = String, Error = ()> + Send {
            Err(()).into_future()
        }

        /// @catch(404)
        fn not_found(&self) -> impl Future<Item = String, Error = io::Error> + Send {
            println!("ZOOOOMG");
            Ok("hello".to_string()).into_future()
        }
    }

    impl Catch {
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
