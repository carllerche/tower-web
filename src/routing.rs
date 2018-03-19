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

/// A set of routes
#[derive(Debug, Default)]
pub struct RouteSet {
    routes: Vec<Route>,
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
            return Some(Match {
                destination: &self.destination,
                condition: &self.condition,
            })
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

// ===== impl RouteSet =====

impl RouteSet {
    /// Create a new, empty, `RouteSet`
    pub fn new() -> RouteSet {
        RouteSet {
            routes: vec![],
        }
    }

    /// Append a route to the route set.
    pub fn push(&mut self, route: Route) {
        self.routes.push(route);
    }

    /// Match a request against a route set
    pub fn test(&self, request: &Request<()>) -> Option<Match> {
        for route in &self.routes {
            if let Some(m) = route.test(request) {
                return Some(m);
            }
        }

        None
    }
}

// ===== impl Match =====

impl<'a> Match<'a> {
    /// Returns the matched destination
    pub fn destination(&self) -> &Destination {
        &*self.destination
    }
}
