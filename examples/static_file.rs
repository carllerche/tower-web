#[macro_use]
extern crate tower_web;
extern crate tokio;

use std::{io, path::PathBuf};
use tokio::{fs::File, prelude::Future};
use tower_web::ServiceBuilder;

#[derive(Clone, Debug)]
pub struct SelfServing;

impl_web! {
    impl SelfServing {
        /// @get("/")
        /// @content_type("plain")
        fn index(&self) -> impl Future<Item = File, Error = io::Error> + Send {
            let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            d.push(file!());
            File::open(d)
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(SelfServing)
        .run(&addr)
        .unwrap();
}
