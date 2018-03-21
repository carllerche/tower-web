use super::Params;

use http::{Method, Request};

/// Requirement on an HTTP request in order to match a route
#[derive(Debug)]
pub struct Condition {
    /// HTTP method used to match the route
    method: Method,

    /// Path used to match the route
    path: String,
}

#[derive(Debug)]
pub struct Match<T> {
    /// Matched routing destination
    destination: T,

    /// Extracted route parameters
    params: Params,
}

#[derive(Debug)]
enum Segment {
    Literal(String),
    Param,
}

// ===== impl Condition =====

impl Condition {
    pub fn new(method: Method, path: &str) -> Condition {
        let path = path.to_string();

        Condition {
            method,
            path
        }
    }

    pub fn test(&self, request: &Request<()>) -> Option<Params> {
        if *request.method() != self.method {
            return None;
        }

        if request.uri().path() != self.path {
            return None;
        }

        unimplemented!();
    }
}

// ===== impl Match =====

impl<T> Match<T> {
    pub(crate) fn new(destination: T,
                      params: Params)
        -> Self
    {
        Match {
            destination,
            params,
        }
    }

    /// Returns the matched destination
    pub fn destination(&self) -> &T {
        &self.destination
    }

    /// Returns the matched parameters
    pub fn params(&self) -> &Params {
        &self.params
    }

    pub(crate) fn into_parts(self) -> (T, Params) {
        (self.destination, self.params)
    }
}
