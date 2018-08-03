use super::{Route, RouteSet};

use std::mem;

pub struct Builder<T> {
    routes: RouteSet<T>,
}

impl<T> Builder<T> {
    pub fn new() -> Self {
        Builder {
            routes: RouteSet::new(),
        }
    }

    /// Insert a route value
    pub fn insert(&mut self, route: Route<T>) -> &mut Self {
        self.routes.insert(route);
        self
    }

    pub(crate) fn insert_all(&mut self, set: RouteSet<T>) -> &mut Self {
        self.routes.insert_all(set);
        self
    }

    pub fn build(&mut self) -> RouteSet<T> {
        mem::replace(&mut self.routes, RouteSet::new())
    }
}
