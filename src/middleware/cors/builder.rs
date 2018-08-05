use super::{AllowedOrigins, Config, CorsMiddleware};
use http::{header::HeaderName, HttpTryFrom, Method};
use std::{collections::HashSet, time::Duration};

// why no httptryinto

use http;

#[derive(Debug, Clone)]
pub struct CorsBuilder {
    config: Config,
}

impl CorsBuilder {
    /// Create a new `CorsBuilder` with default configuration.
    ///
    /// By default, all operations are restricted.
    pub fn new() -> CorsBuilder {
        CorsBuilder {
            config: Config::default(),
        }
    }

    /// Create a new `CorsBuilder` with a permissive configuration.
    pub fn permissive() -> CorsBuilder {
        CorsBuilder {
            config: Config {
                allowed_methods: all_standard_methods(),
                allowed_origins: AllowedOrigins::Any { allow_null: false },
                allowed_headers: Default::default(), // FIXME: ???
                allow_credentials: false,            // FIXME: ???
                exposed_headers: Default::default(), // FIXME: ???
                max_age: None,                       // FIXME: ???
                prefer_wildcard: false,              // FIXME: ???
            },
        }
    }

    pub fn allow_origins(mut self, origins: AllowedOrigins) -> Self {
        self.config.allowed_origins = origins;
        self
    }

    pub fn allow_methods<I>(mut self, methods: I) -> Self
    where
        I: IntoIterator,
        Method: HttpTryFrom<I::Item>,
    {
        let methods = methods
            .into_iter()
            .map(|v| HttpTryFrom::try_from(v).unwrap_or_else(|_| panic!("Invalid allowed method")));
        self.config.allowed_methods.extend(methods);
        self
    }

    pub fn allow_headers<I>(mut self, headers: I) -> Self
    where
        I: IntoIterator,
        HeaderName: HttpTryFrom<I::Item>,
    {
        let headers = headers.into_iter().map(|v| {
            HttpTryFrom::try_from(v).unwrap_or_else(|_| panic!("Invalid allowed header name"))
        });

        self.config.allowed_headers.extend(headers);
        self
    }

    pub fn allow_credentials(mut self, allow_credentials: bool) -> Self {
        self.config.allow_credentials = allow_credentials;
        self
    }

    pub fn expose_headers<I>(mut self, headers: I) -> Self
    where
        I: IntoIterator,
        HeaderName: HttpTryFrom<I::Item>,
    {
        let headers = headers.into_iter().map(|v| {
            HttpTryFrom::try_from(v).unwrap_or_else(|_| panic!("Invalid exposed header name"))
        });
        self.config.exposed_headers.extend(headers);
        self
    }

    pub fn max_age(mut self, max_age: Duration) -> Self {
        self.config.max_age = Some(max_age);
        self
    }

    pub fn prefer_wildcard(mut self, prefer_wildcard: bool) -> Self {
        self.config.prefer_wildcard = prefer_wildcard;
        self
    }

    pub(super) fn into_config(self) -> Config {
        self.config
    }

    /// Build a `CorsMiddleware` instance.
    pub fn build(self) -> CorsMiddleware {
        CorsMiddleware::new(self.into_config())
    }
}

fn all_standard_methods() -> HashSet<http::Method> {
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
    ].into_iter()
    .collect()
}
