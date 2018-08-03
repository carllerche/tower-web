use super::{Route, RouteSet};

use http;

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

    /// Insert a new route into the route set.
    ///
    /// TODO: Clean up route definition
    pub fn route(
        &mut self,
        destination: T,
        method: http::Method,
        path: &str,
    ) -> &mut Self {
        let route = Route::new(destination, method, path);

        self.routes.push(route);
        self
    }

    /// Insert a route value
    pub fn push(&mut self, route: Route<T>) -> &mut Self {
        self.routes.push(route);
        self
    }

    pub fn build(&mut self) -> RouteSet<T> {
        mem::replace(&mut self.routes, RouteSet::new())
    }
}
