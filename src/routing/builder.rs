use super::condition::Condition;
use super::{Route, RouteSet};

use http;

use std::mem;

pub struct Builder<T, U> {
    routes: RouteSet<T, U>,
}

impl<T, U> Builder<T, U> {
    pub fn new() -> Self {
        Builder {
            routes: RouteSet::new(),
        }
    }

    /// Insert a new route into the route set.
    pub fn route(&mut self,
                 destination: T,
                 method: http::Method,
                 path: &str,
                 content_type: Option<U>) -> &mut Self
    {
        let route = Route::new(
            destination,
            Condition::new(method, path),
            content_type);

        self.routes.push(route);
        self
    }

    /// Insert a route value
    pub fn push(&mut self, route: Route<T, U>) -> &mut Self {
        self.routes.push(route);
        self
    }

    pub fn build(&mut self) -> RouteSet<T, U> {
        mem::replace(&mut self.routes, RouteSet::new())
    }
}
