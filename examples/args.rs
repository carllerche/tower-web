/// Web service with handler arguments
///
/// ## Usage
///
/// Run the example:
///
///     cargo run --example args
///
/// Then send a request:
///
///     curl -v http://localhost:8080/

#[macro_use]
extern crate tower_web;
extern crate tokio;

use tower_web::ServiceBuilder;
use tokio::prelude::*;

/// This type will be the web service implementation.
#[derive(Clone, Debug)]
pub struct ArgResource;

impl_web! {
    impl ArgResource {

        // ===== Path arguments =====

        // Arguments can be extracted from the request path. Segments in the
        // path that start with `:` (`:param` in this example) are path parameters.
        //
        // This route will match all of the following paths:
        //
        // * /one/foo
        // * /one/bar
        // * /one/123
        //
        // The handler function is able to get access to the value of the
        // parameter. It specifies a function argument of the same name as the
        // path parameter. When the function is called by `tower-web`, the
        // valuee of the parameter is passed in.
        //
        // The key is to realize that `tower-web` users function argument names
        // to determine how to populate them.
        //
        /// @get("/one/:param")
        /// @content_type("plain")
        fn path_str(&self, param: String) -> Result<String, ()> {
            Ok(format!("We received: {} in the path", param))
        }

        // This route has two path parameters. Both are provided to the
        // function. The function is not *required* to include them in the
        // function argument list.
        //
        /// @get("/two/:foo/:bar")
        /// @content_type("plain")
        fn path_multi_str(&self, foo: String, bar: String) -> Result<String, ()> {
            Ok(format!("We received: {} and {} in the path", foo, bar))
        }

        // The argument type is used to validate the arument. If `:num` is not a
        // valid `u32` value, then the HTTP request will fail and a 400 bad
        // request is returned as a response.
        //
        /// @get("/num/:num")
        /// @content_type("plain")
        fn path_num(&self, num: u32) -> Result<String, ()> {
            Ok(format!("We received: {} in the path", num))
        }

        // ===== Query string arguments =====

        // The HTTP request's query string is accessed by including an argument
        // named `query_string`.
        //
        // The following two requests will succeed:
        //
        // curl -vv http://localhost:8080/query-string?foo
        // curl -vv http://localhost:8080/query-string
        //
        /// @get("/query-string")
        /// @content_type("plain")
        fn hello_query_string(&self, query_string: String) -> Result<String, ()> {
            Ok(format!("We received the query {:?}", query_string))
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(ArgResource)
        .run(&addr)
        .unwrap();
}
