use super::Chain;

use tower_service::Service;

/// Decorates a `Service`, transforming either the request or the response.
///
/// TODO: Should this trait re-export Service associated types?
pub trait Middleware<S: Service> {
    type Request;
    type Response;
    type Error;
    type Service: Service<Request = Self::Request,
                         Response = Self::Response,
                            Error = Self::Error>;

    fn wrap(&self, inner: S) -> Self::Service;

    fn chain<T>(self, middleware: T) -> Chain<S, Self, T>
    where T: Middleware<Self::Service>,
          Self: Sized,
    {
        Chain::new(self, middleware)
    }
}
