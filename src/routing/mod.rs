pub mod set;

pub use self::set::RouteSet;

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
pub struct Match<'a, T> {
    destination: T,
    condition: &'a Condition,
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
        if self.condition.test(request) {
            return Some(Match::new(self.destination.clone(), &self.condition));
        }

        None
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

    pub fn test(&self, request: &Request<()>) -> bool {
        if *request.method() != self.method {
            return false;
        }

        if request.uri().path() != self.path {
            return false;
        }

        true
    }
}

// ===== impl Match =====

impl<'a, T> Match<'a, T> {
    pub(crate) fn new(destination: T, condition: &'a Condition) -> Self {
        Match {
            destination,
            condition,
        }
    }

    /// Returns the matched destination
    pub fn destination(&self) -> &T {
        &self.destination
    }

    pub(crate) fn into_parts(self) -> (T, &'a Condition) {
        (self.destination, self.condition)
    }
}
