use super::{Params, Path};

use http::{Method, Request};

/// Matches an HTTP request with a service funtion.
#[derive(Debug)]
pub struct Route<T> {
    /// Where to route the request
    destination: T,

    /// HTTP method used to match the route
    method: Method,

    /// Path used to match the route
    path: Path,
}

impl<T> Route<T> {
    /// Create a new route
    pub(crate) fn new(destination: T, method: Method, path: &str) -> Self {
        let path = Path::new(path);

        Route {
            destination,
            method,
            path,
        }
    }

    pub(crate) fn map<F, U>(self, f: F) -> Route<U>
    where
        F: Fn(T) -> U,
    {
        let destination = f(self.destination);

        Route {
            destination,
            method: self.method,
            path: self.path,
        }
    }
}

impl<T> Route<T>
where
    T: Clone,
{
    /// Try to match a request against this route.
    pub(crate) fn test<'a>(
        &'a self,
        request: &Request<()>,
    ) -> Option<(T, Params)> {

        if *request.method() != self.method {
            return None;
        }

        self.path.test(request.uri().path())
            .map(|params| {
                (self.destination.clone(), params)
            })
    }
}
