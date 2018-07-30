mod chain;

pub use self::chain::Chain;

use tower_service::Service;

/// Decorates a `Service`, transforming either the request or the response.
pub trait Filter<S: Service> {
    type Service: Service;

    fn wrap(&self, inner: S) -> Self::Service;

    fn chain<T>(self, filter: T) -> Chain<S, Self, T>
    where T: Filter<Self::Service>,
          Self: Sized,
    {
        Chain::new(self, filter)
    }
}
