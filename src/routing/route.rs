use super::condition::Condition;
use super::route_match::RouteMatch;

use http::Request;

/// Matches an HTTP request with a service funtion.
#[derive(Debug)]
pub struct Route<T> {
    /// Where to route the request
    destination: T,

    /// When to match on this route
    condition: Condition,

    /// Content-type produced by the route
    content_type: Option<String>,
}

impl<T> Route<T> {
    /// Create a new route
    pub(crate) fn new(destination: T,
                      condition: Condition,
                      content_type: Option<String>) -> Self
    {
        Route {
            destination,
            condition,
            content_type,
        }
    }

    pub(crate) fn map<F, U>(self, f: F) -> Route<U>
    where
        F: Fn(T) -> U,
    {
        let destination = f(self.destination);

        Route {
            destination,
            condition: self.condition,
            content_type: self.content_type,
        }
    }
}

impl<T: Clone> Route<T> {
    /// Try to match a request against this route.
    pub fn test<'a>(&'a self, request: &'a Request<()>) -> Option<(T, RouteMatch<'a>)> {
        self.condition
            .test(request)
            .map(|params| {
                let content_type = self.content_type.as_ref()
                    .map(|s| &s[..]);

                let route_match = RouteMatch::new(params, content_type);

                (self.destination.clone(), route_match)
            })
    }
}
