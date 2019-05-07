use super::{Middleware, Chain};

use tower_service::Service;

/// A no-op middleware.
///
/// When wrapping a `Service`, the `Identity` middleware returns the provided
/// service without modifying it.
#[derive(Debug, Default, Clone)]
pub struct Identity {
    _p: (),
}

impl Identity {
    /// Create a new `Identity` value
    pub fn new() -> Identity {
        Identity { _p: () }
    }
}

/// Decorates a `Service`, transforming either the request or the response.
impl<S: Service> Middleware<S> for Identity {
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Service = S;

    fn wrap(&self, inner: S) -> Self::Service {
        inner
    }
}

impl<T> crate::util::Chain<T> for Identity {
    type Output = Chain<Self, T>;

    fn chain(self, other: T) -> Self::Output {
        Chain::new(self, other)
    }
}
