use super::condition::Condition;
use super::{Route, RouteSet};

use http;

use std::mem;

pub struct Builder<T> {
    routes: Vec<Route<T>>,
}

impl<T> Builder<T> {
    pub fn new() -> Self {
        Builder { routes: vec![] }
    }

    /// Insert a new route into the route set.
    pub fn route(&mut self, destination: T, method: http::Method, path: &str) -> &mut Self {
        self.routes
            .push(Route::new(destination, Condition::new(method, path)));
        self
    }

    /// Insert a route value
    pub fn push(&mut self, route: Route<T>) -> &mut Self {
        self.routes.push(route);
        self
    }

    pub fn build(&mut self) -> RouteSet<T> {
        let routes = mem::replace(&mut self.routes, vec![]);
        RouteSet::new(routes)
    }
}
