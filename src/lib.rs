//! Tower Web is fast, web framework that aims to remove boilerplate.
//!
//! The goal is to decouple all HTTP concepts from the application logic. You
//! implement your application using "plain old Rust types" and Tower Web uses a
//! macro to generate the necessary glue to serve the application as an HTTP
//! service.
//!
//! The bulk of Tower Web lies in the [`impl_web`] macro. Tower web also
//! provides `#[derive(Extract)]` (for extracting data out of the HTTP request)
//! and `#[derive(Response)]` (for converting a struct to an HTTP response).
//!
//! ## `impl_web`
//!
//! ## `derive(Extract)`
//!
//! ## `derive(Response)`
//!
//! ## Testing
//!
//! [`impl_web`]: macro.impl_web.html
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
