/// Hello world service.
///
/// A simple service that demonstrates how to get started with `tower-web`.
///
/// ## Overview
///
/// `tower-web` lets you define a web service using "plain old Rust types"
/// (PORT). The idea is to decouple all HTTP concepts from the business logic.
/// This is achieved by using macros that inspect the handler functions on your
/// PORT and generate a [Tower `Service`][service] implementation.
///
/// `tower-web` removes most of the boiler plate from implementing web apps by
/// embracing convention over configuration.
///
/// [service]: https://github.com/tower-rs/tower
///
/// ## Service and Resources
///
/// First, some quick terminology.
///
/// A "service" is a type that receives and responds to HTTP requests. A "route"
/// is a specific mapping from an HTTP request to a function handler. A
/// "resource" is a bundle of routes. A "service" is one or more resources.
///
/// For example, a single service might have the following resources:
///
/// * User
/// * Article
/// * Comment
///
/// Each one of those resources would have multiple routes, including, but not
/// limited to:
///
/// * User::list
/// * User::create
/// * User::update
/// * Article::show
/// * Comment::delete
///
/// ## Usage
///
/// Run the example:
///
///     cargo run --example hello_world
///
/// Then send a request:
///
///     curl -v http://localhost:8080/

#[macro_use]
extern crate tower_web;
extern crate tokio;

use tower_web::ServiceBuilder;
use tokio::prelude::*;

/// This type will be part of the web service as a resource.
#[derive(Clone, Debug)]
pub struct HelloWorld {
    /// The message that will be included in the response to `/motd`.
    motd: String,
}

/// To derive `Resource`, the implementation of `HelloWorld` is contained in the
/// `impl_web!` macro. This macro does not modify any of its contents. It will
/// inspect the implementation and, with the help of some annotations, generate
/// the necessary glue code to run the web service.
///
/// impl_web! is a temporary solution to enable tower-web to work with stable
/// rust. In the near future, this will be transitioned to use attribute macros.
impl_web! {
    impl HelloWorld {

        // `hello_world` is a plain old method on `HelloWorld`. However, note
        // that there are some doc comments. `impl_web` looks at these comments
        // and uses them to generate the glue code.
        //
        // Functions must take `&self` and return `T: IntoFuture`. Since
        // `Result<_, _>` implements `IntoFuture`, it is a valid return type.
        // If the function returns `Err`, it will be mapped to an HTTP 500
        // response.
        //
        // #[get("/")] matches GET HTTP requests to `/`.
        //
        #[get("/")]
        fn hello_world(&self) -> Result<&'static str, ()> {
            Ok("This is a basic response served by tower-web")
        }

        // This route match GET `/motd` requests. It uses a field on
        // `HelloWorld` to generate the response.
        //
        #[get("/motd")]
        fn motd(&self) -> Result<String, ()> {
            // You can also respond with an owned `String`.
            Ok(format!("MOTD: {}", self.motd))
        }

        // Here a future is used to generate the response. This allows for
        // asynchronous processing of the request.
        //
        // Note that `impl Future` is bound by `Send`. Currently, hyper
        // requires everything to be `Send`. So, in order to run our service, we
        // also have to guarantee that everything is Send.
        //
        #[get("/hello-future")]
        fn hello_future(&self) -> impl Future<Item = String, Error = ()> + Send {
            future::ok("Or return a future that resolves to the response".to_string())
        }

        // Other HTTP verbs are supported as well.
        //
        #[post("/print_std")]
        fn print_std(&self) -> Result<&'static str, ()> {
            println!("Hello from the web");
            Ok("done")
        }
    }
}

pub fn main() {
    // Next, we must run our web service.
    //
    // The HTTP service will listen on this address and port.
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    // A service builder is used to configure our service.
    ServiceBuilder::new()
        // We add the resources that are part of the service. In this example,
        // there is only a single resource.
        .resource(HelloWorld {
            motd: "tower-web is amazing!!!".to_string(),
        })
        // We run the service
        .run(&addr)
        .unwrap();
}
