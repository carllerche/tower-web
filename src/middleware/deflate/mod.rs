//! Middleware that deflates HTTP response bodies

mod middleware;
mod service;

pub use self::middleware::DeflateMiddleware;
pub use self::service::{DeflateService, ResponseFuture};
