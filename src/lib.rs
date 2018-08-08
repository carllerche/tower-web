//! Tower Web is fast, web framework that aims to remove boilerplate.
//!
//! The goal is to decouple all HTTP concepts from the application logic. You
//! implement your application using "plain old Rust types" and Tower Web uses a
//! macro to generate the necessary glue to serve the application as an HTTP
//! service.
//!
//! The bulk of Tower Web lies in the [`impl_web`] macro. Tower web also
//! provides [`#[derive(Extract)]`][d-ex] (for extracting data out of the HTTP request)
//! and [`#[derive(Response)]`][d-resp] (for converting a struct to an HTTP response).
//!
//! The examples directory contains a number of examples showing how to use Tower
//! Web.
//!
//! [d-ex]: #deriveextract
//! [d-resp]: #deriveresponse
//!
//! ## `impl_web!`
//!
//! The `impl_web!` macro wraps one or more `impl` blocks and generates
//! `Resource` implementations. These structs may then be passed to
//! [`ServiceBuilder`].
//!
//! ```rust
//! # #[macro_use] extern crate tower_web;
//! struct MyApp;
//!
//! impl_web! {
//!     impl MyApp {
//!         /// @get("/")
//!         fn index(&self) -> Result<String, ()> {
//!             // implementation
//! #           unimplemented!()
//!         }
//!     }
//! }
//! ```
//!
//! The `/// @get("/")` is meaningful. This is how `impl_web!` works today on
//! Rust stable. Once attribute macros land on stable, Tower Web will switch to
//! that.
//!
//! `impl_web!` looks for methods that have a [routing] attribute. These methods
//! will be exposed from the web service. All other methods will be ignored.
//!
//! [routing]: #routing
//!
//! ### Routing
//!
//! Routing attributes start with an HTTP verb and contain a path that is
//! matched. For example:
//!
//! * `@get("/")`
//! * `@post("/foo")`
//! * `@put("/zomg/hello/world")`
//!
//! #### Captures
//!
//! Path segments that begin with `:` are captures. They match any path segment
//! and allow the resource method to get access to the value. For example:
//!
//! ```rust
//! # #[macro_use] extern crate tower_web;
//! struct MyApp;
//!
//! impl_web! {
//!     impl MyApp {
//!         /// @get("/hello/:msg")
//!         fn index(&self, msg: String) -> Result<String, ()> {
//!             Ok(format!("Got: {}", msg))
//!         }
//!     }
//! }
//! ```
//!
//! The function argument is named `msg`. The macro will match the argument name
//! with the capture name and call `index`, passing the value captured from the
//! path as the first argument.
//!
//! ### Method Arguments
//!
//! `impl_web!` populates resource method arguments using data from the HTTP
//! request. The name of the argument is important as it tells the macro what
//! part of the request to use. The rules are as follows:
//!
//! * Path captures: when the argument name matches a capture name.
//! * Query string: when the argument is named `query_string`.
//! * Request body: when the argument is named `body`.
//! * All other names are pulled from HTTP headers.
//!
//! The **type** of all method arguments must implement [`Extract`]. So, for a
//! list of possible argument types, see what implements [`Extract`].
//!
//! For example:
//!
//! ```rust
//! # #[macro_use] extern crate tower_web;
//! struct MyApp;
//!
//! impl_web! {
//!     impl MyApp {
//!         /// @get("/path/:capture")
//!         fn index(&self, capture: String, query_string: String) -> Result<String, ()> {
//!             Ok(format!("capture={}; query_string={}", capture, query_string))
//!         }
//!
//!         /// @post("/upload")
//!         fn upload(&self, content_type: String, body: Vec<u8>) -> Result<String, ()> {
//!             // implementation
//! #           unimplemented!()
//!         }
//!     }
//! }
//! ```
//!
//! #### Validation
//!
//! The HTTP request can be validated by specifying an argument type that
//! enforces an invariant. For example, if a path segment must be numeric, the
//! argument should be specified as such:
//!
//! ```rust
//! # #[macro_use] extern crate tower_web;
//! struct MyApp;
//!
//! impl_web! {
//!     impl MyApp {
//!         /// @get("/users/:id")
//!         fn get_user(&self, id: u32) -> Result<String, ()> {
//!             // implementation
//! #           unimplemented!()
//!         }
//!     }
//! }
//! ```
//!
//! In the previous example, requests to `/users/123` will succeed but a request
//! to `/users/foo` will result in a response with a status code of 400 (bad
//! request).
//!
//! `Option` is another useful type for validating the request. If an argument
//! is of type `Option`, the request will not be rejected if the argument is not
//! present. For example:
//!
//! ```rust
//! # #[macro_use] extern crate tower_web;
//! struct MyApp;
//!
//! impl_web! {
//!     impl MyApp {
//!         /// @get("/")
//!         fn get(&self, x_required: String, x_optional: Option<String>) -> Result<String, ()> {
//!             // implementation
//! #           unimplemented!()
//!         }
//!     }
//! }
//! ```
//!
//! In the previous example, requests to `/` **must** provide a `X-Required`
//! heeader, but may (or may not) provide a `X-Optional` header.
//!
//! [`Extract`]: extract/trait.Extract.html
//!
//! ### Return type
//!
//! Resource methods return types are futures yielding items that implement
//! [`Response`]. This includes types like:
//!
//! * [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
//! * [`serde_json::Value`](https://docs.rs/serde_json/1/serde_json/enum.Value.html)
//! * [`http::Response`](https://docs.rs/http/0.1.9/http/response/index.html)
//!
//! The return type is either specified explicitly or `impl Future` can be used:
//!
//! ```rust
//! # #[macro_use] extern crate tower_web;
//! # extern crate futures;
//! # use futures::Future;
//! # type MyResponseFuture = Result<String, ()>;
//! struct MyApp;
//!
//! impl_web! {
//!     impl MyApp {
//!         /// @get("/foo")
//!         fn foo(&self) -> MyResponseFuture {
//!             // implementation
//! #           unimplemented!()
//!         }
//!
//!         /// @get("/bar")
//!         fn bar(&self) -> impl Future<Item = String> + Send {
//!             // implementation
//! #           futures::future::ok::<_, ()>("".to_string())
//!         }
//!     }
//! }
//! ```
//!
//! See the examples directory for more examples on responding to requests.
//!
//! [`Response`]: response/trait.Response.html
//!
//! ### Limitations
//!
//! In order to work on stable Rust, `impl_web!` is implemented using
//! [`proc-macro-hack`], which comes with some [limitations]. The main one being
//! that it can be used only once per scope. This doesn't cause problems in
//! practice multiple resource implementations can be included in a single
//! `impl_web!` clause:
//!
//! ```rust
//! # #[macro_use] extern crate tower_web;
//! # struct Resource1;
//! # struct Resource2;
//! impl_web! {
//!     impl Resource1 {
//!         // impl...
//!     }
//!
//!     impl Resource2 {
//!         // impl...
//!     }
//!
//!     // additional impls
//! }
//! ```
//!
//! ## `derive(Extract)`
//!
//! Using `derive(Extract)` on a struct generates an `Extract` implementation,
//! which enables the struct to be used as an resource method argument.
//!
//! `derive(Extract)` calls [Serde]'s `derive(Deserialize)` internally, so all
//! the various Serde annotations apply here as well. See Serde's documentation
//! for more details on those.
//!
//! ```rust
//! # #[macro_use] extern crate tower_web;
//! struct MyApp;
//!
//! #[derive(Extract)]
//! struct MyData {
//!     foo: String,
//!     bar: u32,
//!     baz: Option<u32>,
//! }
//!
//! impl_web! {
//!     impl MyApp {
//!         /// @get("/")
//!         fn index(&self, query_string: MyData) -> Result<String, ()> {
//!             // implementation
//! #           unimplemented!();
//!         }
//!     }
//! }
//! ```
//!
//! In the previous example, the query string will be deserialized into the
//! `MyQueryString` struct and passed to the resource method. Both `foo` and
//! `bar` are required, but `baz` is not. This means that the following query
//! strings are acceptable:
//!
//! * `?foo=one&bar=2`
//! * `?foo=one&bar=2&baz=3`
//!
//! However, the following query strings will be rejected:
//!
//! * `?foo=one&bar=two`: `bar` must be numeric
//! * `?foo=one`: `bar` is missing.
//!
//! `derive(Extract)` can also be used to deserialize request bodies:
//!
//! ```rust
//! # #[macro_use] extern crate tower_web;
//! struct MyApp;
//!
//! #[derive(Extract)]
//! struct MyData {
//!     foo: String,
//!     bar: u32,
//!     baz: Option<u32>,
//! }
//!
//! impl_web! {
//!     impl MyApp {
//!         /// @post("/data")
//!         fn index(&self, body: MyData) -> Result<String, ()> {
//!             // implementation
//! #           unimplemented!();
//!         }
//!     }
//! }
//! ```
//!
//! This is the same example as earlier, but this time the argument is named
//! `body`. This tells the macro to populate the argument by deserializing the
//! request body. The request body is deserialized into an instance of `MyData`
//! and passed to the resource method.
//!
//! ## `derive(Response)`
//!
//! Using `derive(Response)` on a struct generates a `Response` implementation,
//! which enables the struct to be used as a resource method return type.
//!
//! `derive(Response)` calls [Serde]'s `derive(Serialize)` internally, so all
//! the various Serde annotations apply here as well. See Serde's documentation
//! for more details on those.
//!
//! Tower Web provides some additional functionality on top of Serde. The
//! following annotations can be used with `derive(Response)`
//!
//! * `#[web(status)]`
//! * `#[web(header)]`
//!
//! Using these two attributes allows configuring the HTTP response status code
//! and header set.
//!
//! For example:
//!
//! ```rust
//! # #[macro_use] extern crate tower_web;
//! struct MyApp;
//!
//! #[derive(Response)]
//! #[web(status = "201")]
//! #[web(header(name = "x-foo", value = "bar"))]
//! struct MyData {
//!     foo: String,
//! }
//!
//! impl_web! {
//!     impl MyApp {
//!         /// @post("/data")
//!         fn create(&self) -> Result<MyData, ()> {
//!             // implementation
//! #           unimplemented!();
//!         }
//!     }
//! }
//! ```
//!
//! In the previous example, the HTTP response generated by `create` will have
//! an HTTP status code of 201 and includee the `X-Foo` HTTP header set to
//! "bar".
//!
//! These annotations may also be used to dynamically set the status code and
//! response headers:
//!
//! ```rust
//! # #[macro_use] extern crate tower_web;
//! #[derive(Response)]
//! struct CustomResponse {
//!     #[web(status)]
//!     custom_status: u16,
//!
//!     #[web(header)]
//!     x_foo: &'static str,
//! }
//! ```
//!
//! When responding with `CustomResponse`, the HTTP status code will be set to
//! the value of the `custom_status` field and the `X-Foo` header will be set to
//! thee value of the `x_foo` field.
//!
//! ## Testing
//!
//! [`impl_web`]: macro.impl_web.html
//! [`proc-macro-hack`]: #
//! [limitations]: #
//! [`ServiceBuilder`]: struct.ServiceBuilder.html
//! [Serde]: http://serde.rs/
extern crate atoi;
extern crate bytes;
#[macro_use]
extern crate futures;
extern crate http;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate serde_plain;
extern crate serde_urlencoded;
extern crate tokio;
extern crate tokio_fs;
extern crate tokio_io;
extern crate tower_service;

pub mod codegen;
pub mod error;
pub mod extract;
pub mod middleware;
pub mod response;
pub mod routing;
pub mod service;
pub mod util;

mod run;

pub use error::Error;
pub use service::ServiceBuilder;

// ===== serde_derive re-export =====

#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;

#[doc(hidden)]
pub use serde_derive::*;

// ===== proc_macro_hack junk =====

#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate tower_web_macros;

#[doc(hidden)]
pub use tower_web_macros::*;

proc_macro_item_decl! {
    #[doc(hidden)]
    derive_resource! => derive_resource_impl
}

#[macro_export]
macro_rules! impl_web {
    ($($t:tt)*) => {
        $($t)*
        derive_resource! { $($t)* }
    }
}

// ===== end proc_macro_hack junk =====
