use crate::config::Config;
use crate::routing::Captures;

use http::Request;

/// Data captured from an HTTP request when it matches a route.
///
/// Primarily, this stores the path captures.
///
/// This type is not intended to be used directly.
#[derive(Debug)]
pub struct RouteMatch<'a> {
    /// The matched HTTP request head
    request: &'a Request<()>,

    /// Route captures
    captures: Captures,

    /// Config
    config: &'a Config,
}

impl<'a> RouteMatch<'a> {
    /// Create a new `RouteMatch`
    pub(crate) fn new(request: &'a Request<()>, captures: Captures, config: &'a Config) -> Self {
        RouteMatch {
            request,
            captures,
            config,
        }
    }

    pub(crate) fn request(&self) -> &Request<()> {
        self.request
    }

    pub(crate) fn captures(&self) -> &Captures {
        &self.captures
    }

    pub(crate) fn config(&self) -> &Config {
        &self.config
    }
}
