use super::{Route, Match};

use http::Request;

/// A set of routes
#[derive(Debug, Default)]
pub struct RouteSet<T> {
    routes: Vec<Route<T>>,
}

/// An iterator that moves routes of a `RouteSet`.
#[derive(Debug)]
pub struct IntoIter<T> {
    inner: ::std::vec::IntoIter<Route<T>>,
}

// ===== impl RouteSet =====

impl<T: Clone> RouteSet<T> {
    /// Create a new, empty, `RouteSet`
    pub fn new() -> RouteSet<T> {
        RouteSet {
            routes: vec![],
        }
    }

    /// Append a route to the route set.
    pub fn push(&mut self, route: Route<T>) {
        self.routes.push(route);
    }

    /// Match a request against a route set
    pub fn test(&self, request: &Request<()>) -> Option<Match<T>> {
        for route in &self.routes {
            if let Some(m) = route.test(request) {
                return Some(m);
            }
        }

        None
    }
}

impl<T> IntoIterator for RouteSet<T> {
    type Item = Route<T>;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let inner = self.routes.into_iter();
        IntoIter { inner }
    }
}

// ===== impl IntoIter =====

impl<T> Iterator for IntoIter<T> {
    type Item = Route<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
