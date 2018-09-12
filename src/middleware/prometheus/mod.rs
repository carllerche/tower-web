//! Middleware that creates a log entry for each HTTP request.

mod middleware;
mod service;

pub use self::middleware::PrometheusMiddleware;
pub use self::service::{PrometheusService, ResponseFuture};
