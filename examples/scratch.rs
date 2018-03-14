#[macro_use]
extern crate tower_web;

#[macro_use]
extern crate serde_derive;

use tower_web::*;

#[derive(Clone, Debug)]
pub struct HelloWorld;

#[derive(Debug, Serialize)]
pub struct HelloResponse {
    msg: &'static str,
}

#[derive(Clone, Debug)]
pub struct OtherResource;

impl_web! {
    // #[content_type("json")]
    impl HelloWorld {
        #[GET "/"]
        // #[GET("/")]
        fn hello_world(&mut self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse {
                msg: "hello world",
            })
        }

        /*
        #[GET("/stream")]
        #[streaming]
        fn stream_hello(&mut self) -> Result<Receiver<String>, ()> {
            unimplemented!();
        }

        #[GET("/raw")]
        #[raw]
        fn raw_response(&mut self) -> Result<Vec<u8>, ()> {
            unimplemented!();
        }

        #[GET("/template")]
        fn template_hello(&mut self) -> Result<Template, ()> {
            unimplemented!();
        }
        */
    }

    /*
    impl OtherResource {
    }
    */
}

/*
trait Resource {
    fn call(&mut self, request: http::Request<()>, ctx: &Context) -> Result<Return, ()>;
}

trait Response {
    fn into_http(self, request: &http::Request<()>, ctx: &Context) -> http::Response<()>;
}

/// Serde types get serialized via serde
impl<T: Serialize> Response for T {}

/// Raw types do not get serialized, but are passed through
impl Response for Raw {}

/// Streaming types are streaming yo
impl Response for Streaming {}
*/

/*
 * String => plain / json
 * Vec<u8> => plain / yeah...
 * Template => ???
 * Streaming => T: Stream<???>
 * T: Serialize => ??
 */

pub fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();

    ServiceBuilder::new()
        /*
        .accept("json")
        .content_type("json")
        */
        .resource(HelloWorld)
        // .resource(OtherResource)
        .run(&addr)
        .unwrap();

    // tower_web::run(&addr, HelloWorld).unwrap();
}
