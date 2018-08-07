/// Web service that responds using static files from disk.
///
/// ## Overview
///
/// Resources may respond with any type that implements the `Response` trait.
/// Tower Web provides an implementation of `Response` for `tokio::fs::File`.
/// So, to use a static file as an HTTP response, the resource return type is
/// set to `tokio::fs::File`.
///
/// ## Usage
///
/// Run the example:
///
///     cargo run --example static_file
///
/// Then send a request:
///
///     curl -v http://localhost:8080/

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

        // This is an example of what **not** to do.
        //
        // While a glob can be extracted from the path as a `String`, this will
        // ofer no protection against path traversal attacks.
        //
        // See below for the correct way to do it.
        //
        /// @get("/unsafe-files/*relative_path")
        /// @content_type("plain")
        fn unsafe_files(&self, relative_path: String) -> impl Future<Item = File, Error = io::Error> + Send {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            // String does no checks for path traversal; do not use this in production!
            path.push(relative_path);
            File::open(path)
        }

        // Tower Web extracts to `PathBuf` by ensuring that "../" is safely
        // rejected. This prevents an attacker from accessing files outside of
        // the "public" directory.
        //
        /// @get("/files/*relative_path")
        /// @content_type("plain")
        fn files(&self, relative_path: PathBuf) -> impl Future<Item = File, Error = io::Error> + Send {
            let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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
