use super::condition::{Condition, Match};

use http::Request;

/// Matches an HTTP request with a service funtion.
#[derive(Debug)]
pub struct Route<T> {
    /// Where to route the request
    destination: T,

    /// When to match on this route
    condition: Condition,
}

impl<T> Route<T> {
    /// Create a new route
    pub(crate) fn new(destination: T, condition: Condition) -> Self {
        Route {
            destination,
            condition,
        }
    }

    pub(crate) fn map<F, U>(self, f: F) -> Route<U>
    where F: Fn(T) -> U
    {
        let destination = f(self.destination);

        Route {
            destination,
            condition: self.condition,
        }
    }
}

impl<T: Clone> Route<T> {
    /// Try to match a request against this route.
    pub fn test(&self, request: &Request<()>) -> Option<Match<T>> {
        self.condition.test(request)
            .map(|params| {
                Match::new(self.destination.clone(), params)
            })
    }
}
