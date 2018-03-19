pub mod set;

pub use self::set::RouteSet;

use http::{Method, Request};

/// Matches an HTTP request with a service funtion.
#[derive(Debug)]
pub struct Route {
    /// Where to route the request
    destination: Destination,

    /// When to match on this route
    condition: Condition,
}

/// The destination to route a request
#[derive(Debug)]
pub struct Destination(usize);

/// Requirement on an HTTP request in order to match a route
#[derive(Debug)]
pub struct Condition {
    /// HTTP method used to match the route
    method: Method,

    /// Path used to match the route
    path: String,
}

#[derive(Debug)]
pub struct Match<'a> {
    destination: &'a Destination,
    condition: &'a Condition,
}

// ===== impl Route =====

impl Route {
    /// Create a new route
    pub fn new(destination: Destination, condition: Condition) -> Route {
        Route {
            destination,
            condition,
        }
    }

    /// Break up the route into the destination and condition
    pub fn into_parts(self) -> (Destination, Condition) {
        (self.destination, self.condition)
    }

    /// Try to match a request against this route.
    pub fn test(&self, request: &Request<()>) -> Option<Match> {
        if self.condition.test(request) {
            return Some(Match::new(&self.destination, &self.condition));
        }

        None
    }
}

// ===== impl Destination =====

impl Destination {
    /// Create a new destination
    pub fn new(id: usize) -> Destination {
        Destination(id)
    }

    /// Returns the destination's identifier
    pub fn id(&self) -> usize {
        self.0
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

impl<'a> Match<'a> {
    pub fn new(destination: &'a Destination, condition: &'a Condition) -> Match<'a> {
        Match {
            destination,
            condition,
        }
    }

    /// Returns the matched destination
    pub fn destination(&self) -> &Destination {
        &*self.destination
    }

    pub fn into_parts(&self) -> (&'a Destination, &'a Condition) {
        (self.destination, self.condition)
    }
}
