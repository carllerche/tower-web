use super::Chain;

use tower_service::Service;

/// Decorates a `Service`, transforming either the request or the response.
pub trait Middleware<S: Service> {
    type Service: Service;

    fn wrap(&self, inner: S) -> Self::Service;

    fn chain<T>(self, middleware: T) -> Chain<S, Self, T>
    where T: Middleware<Self::Service>,
          Self: Sized,
    {
        Chain::new(self, middleware)
    }
}
