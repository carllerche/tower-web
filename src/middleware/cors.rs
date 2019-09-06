//! CORS middleware.

mod builder;
mod config;
mod middleware;
mod service;

pub use self::builder::CorsBuilder;
pub use self::config::AllowedOrigins;
pub use self::middleware::CorsMiddleware;
pub use self::service::CorsService;

use self::config::{Config, CorsResource};
