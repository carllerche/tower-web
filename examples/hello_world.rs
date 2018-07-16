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
        /*
        /// @get("/")
        /// @content_type("json")
        fn hello_world(&self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse {
                msg: "hello world",
            })
        }
        */

        /// @get("/users/:id")
        /// @content_type("plain")
        fn get(&self, id: u32) -> Result<String, ()> {
            println!("GOT: id={:?};", id);
            Ok("ZOOOOMG YO\n".to_string())
        }

        /*
        /// @get("/whoami")
        /// @content_type("plain")
        fn who_am_i(&mut self, user_agent: String) -> Result<String, ()> {
            Ok(format!("You are: {}\n", user_agent))
        }

        /// @get("/custom-header")
        /// @content_type("json")
        fn custom_header(&mut self, x_foo: &str) -> Result<String, ()> {
            Ok(format!("Sent: {}", x_foo))
        }
        */

        /*
        /// @get("/foo")
        /// @content_type("plain")
        fn qs(&mut self) -> Result<String, ()> {
        }
        */

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
        /// @content_type("plain")
        fn goodbye(&self) -> Result<String, ()> {
            Ok("Other resource".to_string())
            /*
            Ok(HelloResponse {
                msg: "other resource",
            })
            */
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
