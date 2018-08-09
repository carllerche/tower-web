//! HTTP service, middleware, and future utilities
//!
//! This module provides specialized versions of the various tower traits that
//! are easier to work with in the context of HTTP requests and responses.
//!
//! These traits can be used as aliases of sorts. Ideally, Rust will add trait
//! alias support in the language and these traits can go away.
//!
//! * [`HttpService`]: A `Service` of `http::Request` to `http::Response`.
//! * [`HttpMiddleware`]: Middleware for `HttpService`.
//! * [`HttpFuture`]: A future yielding an http::Response, i.e. an `HttpService`
//! response future.
//!
//! These types will (probably) be moved into tower-http.
//!
//! [`HttpService`]: trait.HttpService.html
//! [`HttpMiddleware`]: trait.HttpMiddleware.html
//! [`HttpFuture`]: trait.HttpFuture.html

mod future;
mod middleware;
mod new_service;
mod service;

pub use self::future::{HttpFuture, LiftFuture};
pub use self::middleware::HttpMiddleware;
pub use self::new_service::NewHttpService;
pub use self::service::{HttpService, LiftService};

pub(crate) use self::future::sealed::Sealed as SealedFuture;
