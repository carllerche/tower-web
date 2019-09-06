//! Middleware that creates a log entry for each HTTP request.

mod middleware;
mod service;

pub use self::middleware::LogMiddleware;
pub use self::service::{LogService, ResponseFuture};
