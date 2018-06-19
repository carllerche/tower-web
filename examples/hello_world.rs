#[macro_use]
extern crate tower_web;

#[macro_use]
extern crate serde_derive;

use tower_web::ServiceBuilder;

#[derive(Clone, Debug)]
pub struct HelloWorld;

#[derive(Clone, Debug)]
pub struct GoodbyeWorld;

#[derive(Debug, Serialize)]
pub struct HelloResponse {
    msg: &'static str,
}

#[derive(Debug, Serialize)]
pub struct User {
    id: usize,
}

impl_web! {
    impl HelloWorld {
        /// Hello world endpoint
        ///
        /// @get("/")
        fn hello_world(&mut self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse {
                msg: "hello world",
            })
        }

        /// @get("/users/:id")
        /// @content_type("plain")
        fn get(&mut self, id: u32) -> Result<String, ()> {
            println!("GOT: id={:?};", id);
            Ok("ZOOOOMG YO\n".to_string())
        }

        /*
        /// @GET("/authx")
        fn get(&mut self, content_length: HeaderValue) -> Result<String, ()> {
            unimplemented!();
        }
         */

        /*
        // #[GET "/"]
        fn hello_world(&mut self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse {
                msg: "hello world",
            })
        }
        */

        /*
        /// # Web
        ///
        /// @GET "/users/:id"
        /// @arg content_length = Header("Content-Length")
        #[GET "/users/:id"]
        fn get(&mut self, id: usize) -> Result<User, ()> {
            Ok(User {
                id: 1,
            })
        }
        */
    }

    impl GoodbyeWorld {
        /// @get("/goodbye")
        fn goodbye(&mut self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse {
                msg: "other resource",
            })
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();

    ServiceBuilder::new()
        .resource(HelloWorld)
        .resource(GoodbyeWorld)
        .run(&addr)
        .unwrap();
}
