use super::condition::Condition;
use super::route_match::RouteMatch;

use http::Request;

/// Matches an HTTP request with a service funtion.
#[derive(Debug)]
pub struct Route<T, U> {
    /// Where to route the request
    destination: T,

    /// When to match on this route
    condition: Condition,

    /// Content-type produced by the route
    content_type: Option<U>,
}

impl<T, U> Route<T, U> {
    /// Create a new route
    pub(crate) fn new(destination: T,
                      condition: Condition,
                      content_type: Option<U>) -> Self
    {
        Route {
            destination,
            condition,
            content_type,
        }
    }

    pub(crate) fn map<F, T2>(self, f: F) -> Route<T2, U>
    where
        F: Fn(T) -> T2,
    {
        let destination = f(self.destination);

        Route {
            destination,
            condition: self.condition,
            content_type: self.content_type,
        }
    }
}

impl<T, U> Route<T, U>
where T: Clone,
      U: Clone
{
    /// Try to match a request against this route.
    pub(crate) fn test<'a>(&'a self, request: &'a Request<()>) -> Option<(T, Option<U>, RouteMatch<'a>)> {
        self.condition
            .test(request)
            .map(|params| {
                let content_type = self.content_type.clone();
                let route_match = RouteMatch::new(params);

                (self.destination.clone(), content_type, route_match)
            })
    }
}
