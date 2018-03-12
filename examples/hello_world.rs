#[macro_use]
extern crate tower_web;

pub struct HelloWorld;

impl_web! {
    impl HelloWorld {
        #[GET "/"]
        pub fn hello_world(&mut self) -> Result<&'static str, ()> {
            Ok("hello world")
        }
    }
}

pub fn main() {
    let mut service = HelloWorld;
    println!("HELLO WORLD: {:?}", service.hello_world());
}
