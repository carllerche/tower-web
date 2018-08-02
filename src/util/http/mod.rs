//! HTTP service utilities
//!
//! These types will (probably) be moved into tower-http.

mod future;
mod middleware;
mod new_service;
mod service;

pub use self::future::{HttpFuture, LiftFuture};
pub use self::middleware::HttpMiddleware;
pub use self::new_service::NewHttpService;
pub use self::service::{HttpService, LiftService};

pub(crate) use self::future::sealed::Sealed as SealedFuture;
