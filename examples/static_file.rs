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
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            path.push(file!());
            File::open(path)
        }

        /// @get("/unsafe-files/*relative_path")
        /// @content_type("plain")
        fn unsafe_files(&self, relative_path: String) -> impl Future<Item = File, Error = io::Error> + Send {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            // String does no checks for path traversal; do not use this in production!
            path.push(relative_path);
            File::open(path)
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
