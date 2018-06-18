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
    pub fn route(&mut self,
                 destination: T,
                 method: http::Method,
                 path: &str,
                 content_type: Option<&str>) -> &mut Self
    {
        let route = Route::new(
            destination,
            Condition::new(method, path),
            content_type.map(|s| s.to_string()));

        self.routes.push(route);
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
