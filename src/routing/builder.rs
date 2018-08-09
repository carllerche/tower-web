use super::{Route, RouteSet};

use std::mem;

/// Build a set of routes
#[derive(Debug)]
pub struct Builder<T> {
    routes: RouteSet<T>,
}

impl<T> Builder<T> {
    /// Create a new `Builder` instance
    pub fn new() -> Self {
        Builder {
            routes: RouteSet::new(),
        }
    }

    /// Insert a route into the route set
    pub fn insert(&mut self, route: Route<T>) -> &mut Self {
        self.routes.insert(route);
        self
    }

    pub(crate) fn insert_all(&mut self, set: RouteSet<T>) -> &mut Self {
        self.routes.insert_all(set);
        self
    }

    /// Finalize the route set
    pub fn build(&mut self) -> RouteSet<T> {
        mem::replace(&mut self.routes, RouteSet::new())
    }
}
