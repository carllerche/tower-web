#[macro_use]
extern crate tower_web;
extern crate futures;

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
    let mut service = HelloWorld;
    println!("HELLO WORLD: {:?}", service.hello_world());
}
