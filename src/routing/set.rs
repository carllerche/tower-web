use super::{Route, Params};

use http::Request;

/// A set of routes
#[derive(Debug)]
pub struct RouteSet<T> {
    routes: Vec<Route<T>>,
}

/// An iterator that moves routes of a `RouteSet`.
#[derive(Debug)]
pub struct IntoIter<T> {
    inner: ::std::vec::IntoIter<Route<T>>,
}

// ===== impl RouteSet =====

impl<T> RouteSet<T> {
    pub fn new() -> RouteSet<T> {
        RouteSet { routes: vec![] }
    }

    pub(crate) fn push(&mut self, route: Route<T>) {
        self.routes.push(route);
    }
}

impl<T> RouteSet<T>
where
    T: Clone,
{
    /// Match a request against a route set
    pub(crate) fn test(&self, request: &Request<()>) -> Option<(T, Params)> {
        self.routes
            .iter()
            .flat_map(|route| route.test(request))
            .next()
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
