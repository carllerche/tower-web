#![doc(html_root_url = "https://docs.rs/tower-web/0.2.2")]
#![deny(missing_debug_implementations)]
#![cfg_attr(test, deny(warnings))]

//! Tower Web is a fast web framework that aims to remove boilerplate.
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
//!         #[get("/")]
//!         fn index(&self) -> Result<String, ()> {
//!             // implementation
//! #           unimplemented!()
//!         }
//!     }
//! }
//! ```
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
//! * `#[get("/")]`
//! * `#[post("/foo")]`
//! * `#[put("/zomg/hello/world")]`
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
//!         #[get("/hello/:msg")]
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
//!         #[get("/path/:capture")]
//!         fn index(&self, capture: String, query_string: String) -> Result<String, ()> {
//!             Ok(format!("capture={}; query_string={}", capture, query_string))
//!         }
//!
//!         #[post("/upload")]
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
//!         #[get("/users/:id")]
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
//!         #[get("/")]
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
//!         #[get("/foo")]
//!         fn foo(&self) -> MyResponseFuture {
//!             // implementation
//! #           unimplemented!()
//!         }
//!
//!         #[get("/bar")]
//!         fn bar(&self) -> impl Future<Item = String> + Send {
//!             // implementation
//! #           futures::future::ok::<_, ()>("".to_string())
//!         }
//!     }
//! }
//! ```
//!
//! Note that `impl Future` is bound by `Send`. Hyper currently requires `Send`
//! on all types. So, in order for our service to run with Hyper, we also need
//! to ensure that everything is bound by `Send`.
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
//!         #[get("/")]
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
//!         #[post("/data")]
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
//!         #[post("/data")]
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
//! the value of the `x_foo` field.
//!
//! When a handler can return unrelated response types, like a file or a web
//! page, `derive(Response)` can delegate the `Response` implementation to them,
//! through an enum:
//!
//! ```rust
//! # #[macro_use] extern crate tokio;
//! # #[macro_use] extern crate tower_web;
//! #[derive(Response)]
//! #[web(either)]
//! enum FileOrPage {
//!     File(tokio::fs::File),
//!     Page(String),
//! }
//! ```
//!
//! The `web(either)` attribute is only supported on enums whose variants
//! a single unnamed field. Right now, the other `web` attributes have no effect
//! when using `web(either)`.
//!
//! ## Starting a server
//!
//! Once `Resource` implementations are generated, the types may be passed to
//! [`ServiceBuilder::resource`] in order to define the web service.
//!
//! ```rust
//! # #[macro_use] extern crate tower_web;
//! # use tower_web::ServiceBuilder;
//! # struct Resource1;
//! # struct Resource2;
//! # impl_web! {
//! #     impl Resource1 {}
//! #     impl Resource2 {}
//! # }
//!
//! let addr = "127.0.0.1:8080".parse().expect("Invalid address");
//! println!("Listening on http://{}", addr);
//!
//! # if false {
//! // A service builder is used to configure our service.
//! ServiceBuilder::new()
//!     // We add the resources that are part of the service.
//!     .resource(Resource1)
//!     .resource(Resource2)
//!     // We run the service
//!     .run(&addr)
//!     .unwrap();
//! # }
//! ```
//!
//! ## Testing
//!
//! Because web services build with Tower Web are "plain old Rust types"
//! (PORT?), testing a method is done the exact same way you would test any
//! other rust code.
//!
//! ```rust
//! # #[macro_use] extern crate tower_web;
//! struct MyApp;
//!
//! impl_web! {
//!     impl MyApp {
//!         #[get("/:hello")]
//!         fn index(&self, hello: String) -> Result<&'static str, ()> {
//!             if hello == "hello" {
//!                 Ok("correct")
//!             } else {
//!                 Ok("nope")
//!             }
//!         }
//!     }
//! }
//!
//! #[test]
//! fn test_my_app() {
//!     let app = MyApp;
//!
//!     assert_eq!(app.index("hello".to_string()), Ok("correct"));
//!     assert_eq!(app.index("not-hello".to_string()), Ok("nope"));
//! }
//! ```
//!
//! [`impl_web`]: macro.impl_web.html
//! [`proc-macro-hack`]: #
//! [limitations]: #
//! [`ServiceBuilder`]: struct.ServiceBuilder.html
//! [`ServiceBuilder::resource`]: struct.ServiceBuilder.html#method.resource
//! [Serde]: http://serde.rs/
extern crate atoi;
extern crate bytes;
extern crate checked;
extern crate chrono;
extern crate flate2;
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
extern crate void;

pub mod codegen;
pub mod config;
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

// ===== end proc_macro_hack junk =====

/// Generate a `Resource` implemeentation based on the methods defined in the
/// macro block.
///
/// See [library level documentation](index.html) for more details.
///
/// # Examples
/// ```rust
/// # #[macro_use] extern crate tower_web;
/// struct MyApp;
///
/// impl_web! {
///     impl MyApp {
///         #[get("/")]
///         fn index(&self) -> Result<String, ()> {
///             // implementation
/// #           unimplemented!()
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_web {
    ($($t:tt)*) => {
        impl_web_clean_top_level!(() $($t)*);
        derive_resource!($($t)*);
    }
}

// Tt-muncher to invoke `impl_web_clean_nested!` on the content of every set of
// curly braces in the input.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_web_clean_top_level {
    // Next token is a set of curly braces. Pass to `impl_web_clean_nested!`.
    (($($done:tt)*) { $($nested:tt)* } $($rest:tt)*) => {
        impl_web_clean_nested!(($($done)*) () { $($nested)* } { $($nested)* } $($rest)*);
    };

    // Next token is not a set of curly braces. Keep it.
    (($($done:tt)*) $t:tt $($rest:tt)*) => {
        impl_web_clean_top_level!(($($done)* $t) $($rest)*);
    };

    // No more tokens to process. Expand to the cleaned tokens.
    (($($done:tt)*)) => {
        $($done)*
    };
}

// Tt-muncher to strip tower-web attributes from the input.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_web_clean_nested {
    // Match an attribute that we recognize and discard it.
    (($($outer:tt)*) ($($done:tt)*) { #[get $($attr:tt)*] $($nested:tt)* } $dup:tt $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)*) { $($nested)* } { $($nested)* } $($rest)*);
    };
    (($($outer:tt)*) ($($done:tt)*) { #[post $($attr:tt)*] $($nested:tt)* } $dup:tt $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)*) { $($nested)* } { $($nested)* } $($rest)*);
    };
    (($($outer:tt)*) ($($done:tt)*) { #[put $($attr:tt)*] $($nested:tt)* } $dup:tt $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)*) { $($nested)* } { $($nested)* } $($rest)*);
    };
    (($($outer:tt)*) ($($done:tt)*) { #[patch $($attr:tt)*] $($nested:tt)* } $dup:tt $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)*) { $($nested)* } { $($nested)* } $($rest)*);
    };
    (($($outer:tt)*) ($($done:tt)*) { #[delete $($attr:tt)*] $($nested:tt)* } $dup:tt $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)*) { $($nested)* } { $($nested)* } $($rest)*);
    };
    (($($outer:tt)*) ($($done:tt)*) { #[content_type $($attr:tt)*] $($nested:tt)* } $dup:tt $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)*) { $($nested)* } { $($nested)* } $($rest)*);
    };
    (($($outer:tt)*) ($($done:tt)*) { #[catch $($attr:tt)*] $($nested:tt)* } $dup:tt $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)*) { $($nested)* } { $($nested)* } $($rest)*);
    };

    // Seek forward to the next `#` token. This reduces the depth of our macro
    // recursion to avoid requiring a higher recursion limit for simple
    // invocations.
    (($($outer:tt)*) ($($done:tt)*) { $A:tt # $($nested:tt)* } { $a:tt $pound:tt $($dup:tt)* } $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)* $A) { $pound $($nested)* } { $($nested)* } $($rest)*);
    };
    (($($outer:tt)*) ($($done:tt)*) { $A:tt $B:tt # $($nested:tt)* } { $a:tt $b:tt $pound:tt $($dup:tt)* } $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)* $A $B) { $pound $($nested)* } { $($nested)* } $($rest)*);
    };
    (($($outer:tt)*) ($($done:tt)*) { $A:tt $B:tt $C:tt # $($nested:tt)* } { $a:tt $b:tt $c:tt $pound:tt $($dup:tt)* } $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)* $A $B $C) { $pound $($nested)* } { $($nested)* } $($rest)*);
    };
    (($($outer:tt)*) ($($done:tt)*) { $A:tt $B:tt $C:tt $D:tt # $($nested:tt)* } { $a:tt $b:tt $c:tt $d:tt $pound:tt $($dup:tt)* } $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)* $A $B $C $D) { $pound $($nested)* } { $($nested)* } $($rest)*);
    };
    (($($outer:tt)*) ($($done:tt)*) { $A:tt $B:tt $C:tt $D:tt $E:tt # $($nested:tt)* } { $a:tt $b:tt $c:tt $d:tt $e:tt $pound:tt $($dup:tt)* } $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)* $A $B $C $D $E) { $pound $($nested)* } { $($nested)* } $($rest)*);
    };
    (($($outer:tt)*) ($($done:tt)*) { $A:tt $B:tt $C:tt $D:tt $E:tt $F:tt # $($nested:tt)* } { $a:tt $b:tt $c:tt $d:tt $e:tt $f:tt $pound:tt $($dup:tt)* } $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)* $A $B $C $D $E $F) { $pound $($nested)* } { $($nested)* } $($rest)*);
    };

    // Next several tokens are not part of a tower-web attribute. Keep them.
    (($($outer:tt)*) ($($done:tt)*) { $A:tt $B:tt $C:tt $D:tt $E:tt $F:tt $G:tt $($nested:tt)* } $dup:tt $($rest:tt)*) => {
        impl_web_clean_nested!(($($outer)*) ($($done)* $A $B $C $D $E $F $G) { $($nested)* } { $($nested)* } $($rest)*);
    };

    // Reached the end of nested tokens. Return back to `impl_web_clean_top_level!`.
    (($($outer:tt)*) ($($done:tt)*) { $($nested:tt)* } $dup:tt $($rest:tt)*) => {
        impl_web_clean_top_level!(($($outer)* { $($done)* $($nested)* }) $($rest)*);
    };
}
