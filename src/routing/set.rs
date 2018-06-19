use super::{Route, RouteMatch};

use http::Request;

/// A set of routes
#[derive(Debug)]
pub struct RouteSet<T, U> {
    routes: Vec<Route<T, U>>,
}

/// An iterator that moves routes of a `RouteSet`.
#[derive(Debug)]
pub struct IntoIter<T, U> {
    inner: ::std::vec::IntoIter<Route<T, U>>,
}

// ===== impl RouteSet =====

impl<T, U> RouteSet<T, U> {
    pub fn new() -> RouteSet<T, U> {
        RouteSet {
            routes: vec![],
        }
    }

    pub(crate) fn push(&mut self, route: Route<T, U>) {
        self.routes.push(route);
    }

    /*
    /// Create a new, empty, `RouteSet`
    pub(crate) fn new(routes: Vec<Route<T, U>>) -> RouteSet<T, U> {
        RouteSet { routes }
    }
    */
}

impl<T, U> RouteSet<T, U>
where T: Clone,
      U: Clone,
{
    /// Match a request against a route set
    pub(crate) fn test<'a>(&'a self, request: &'a Request<()>) -> Option<(T, Option<U>, RouteMatch<'a>)> {
        for route in &self.routes {
            if let Some(m) = route.test(request) {
                return Some(m);
            }
        }

        None
    }
}

impl<T, U> IntoIterator for RouteSet<T, U> {
    type Item = Route<T, U>;
    type IntoIter = IntoIter<T, U>;

    fn into_iter(self) -> Self::IntoIter {
        let inner = self.routes.into_iter();
        IntoIter { inner }
    }
}

// ===== impl IntoIter =====

impl<T, U> Iterator for IntoIter<T, U> {
    type Item = Route<T, U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
