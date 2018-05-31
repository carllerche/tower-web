#[macro_use]
extern crate tower_web;

use tower_web::*;

#[derive(Clone, Debug)]
pub struct HelloWorld;

impl_web! {
    impl HelloWorld {
        /// @GET("/")
        fn hello_world(&mut self) -> Result<String, ()> {
            unimplemented!();
        }
    }
}

fn main() {
    println!("Hello, world!");
}
