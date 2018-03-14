#[macro_use]
extern crate tower_web;
extern crate futures;

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct HelloWorld;

impl_web! {
    impl HelloWorld {
        #[GET "/"]
        fn hello_world(&mut self) -> Result<String, ()> {
            Ok("hello world".into())
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();

    tower_web::run(&addr, Arc::new(|| Ok::<_, ()>(HelloWorld))).unwrap();
}
