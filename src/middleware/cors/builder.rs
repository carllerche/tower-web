use super::{CorsMiddleware, Config};

use http;

#[derive(Debug)]
pub struct CorsBuilder {
    config: Config,
}

impl CorsBuilder {
    /// Create a new `CorsBuilder` with default configuration.
    ///
    /// By default, all operations are restricted.
    pub fn new() -> CorsBuilder {
        CorsBuilder {
            config: Config {
                allowed_methods: vec![],
            },
        }
    }

    /// Create a new `CorsBuilder` with a permissive configuration.
    pub fn permissive() -> CorsBuilder {
        CorsBuilder {
            config: Config {
                allowed_methods: all_std_methods(),
            },
        }
    }

    /// Build a `CorsMiddleware` instance.
    pub fn build(&self) -> CorsMiddleware {
        CorsMiddleware::new(self.config.clone())
    }
}

fn all_std_methods() -> Vec<http::Method> {
    use http::Method;

    vec![
        Method::OPTIONS,
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::DELETE,
        Method::HEAD,
        Method::TRACE,
        Method::CONNECT,
        Method::PATCH,
    ]
}
