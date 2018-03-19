use super::{Route, Match};

use http::Request;

/// A set of routes
#[derive(Debug, Default)]
pub struct RouteSet {
    routes: Vec<Route>,
}

/// An iterator that moves routes of a `RouteSet`.
#[derive(Debug)]
pub struct IntoIter {
    inner: ::std::vec::IntoIter<Route>,
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

impl IntoIterator for RouteSet {
    type Item = Route;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        let inner = self.routes.into_iter();
        IntoIter { inner }
    }
}

// ===== impl IntoIter =====

impl Iterator for IntoIter {
    type Item = Route;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
