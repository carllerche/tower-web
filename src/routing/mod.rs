pub mod set;

pub use self::set::RouteSet;

use bytes::Bytes;
use http::{Method, Request};

/// Matches an HTTP request with a service funtion.
#[derive(Debug)]
pub struct Route<T> {
    /// Where to route the request
    destination: T,

    /// When to match on this route
    condition: Condition,
}

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
pub struct Params {
    /// Parameters extracted from the requet
    ///
    /// TODO: More efficient extraction.
    params: Vec<Bytes>,
}

#[derive(Debug)]
enum Segment {
    Literal(String),
    Param,
}

// ===== impl Route =====

impl<T: Clone> Route<T> {
    /// Create a new route
    pub fn new(destination: T, condition: Condition) -> Self {
        Route {
            destination,
            condition,
        }
    }

    /// Try to match a request against this route.
    pub fn test(&self, request: &Request<()>) -> Option<Match<T>> {
        self.condition.test(request)
            .map(|params| {
                Match::new(self.destination.clone(), params)
            })
    }

    pub(crate) fn map<F, U>(self, f: F) -> Route<U>
    where F: Fn(T) -> U
    {
        let destination = f(self.destination);

        Route {
            destination,
            condition: self.condition,
        }
    }
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

// ===== impl Params =====

impl Params {
    /// Get a parameter value
    pub fn get(&self, index: usize) -> &[u8] {
        self.params[index].as_ref()
    }
}
