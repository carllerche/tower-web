use error::Catch;
use routing::{Resource, RoutedService};
use service::WebService;
use util::Never;
use util::http::{HttpMiddleware};

use futures::future::{self, FutureResult};
use http;
use tower_service::NewService;

/// Creates new `WebService` values.
///
/// Instances of this type are created by `ServiceBuilder`. A `NewWebService`
/// instance is used to generate a `WebService` instance per connection.
pub struct NewWebService<T, U, M>
where
    T: Resource,
{
    /// The routed service. This service implements `Clone`.
    service: RoutedService<T, U>,

    /// Middleware to wrap the routed service with
    middleware: M,
}

impl<T, U, M> NewWebService<T, U, M>
where
    T: Resource,
    U: Catch,
    M: HttpMiddleware<RoutedService<T, U>>,
{
    /// Create a new `NewWebService` instance.
    pub(crate) fn new(service: RoutedService<T, U>, middleware: M) -> Self {
        NewWebService {
            service,
            middleware,
        }
    }
}

impl<T, U, M> NewService for NewWebService<T, U, M>
where
    T: Resource,
    U: Catch,
    M: HttpMiddleware<RoutedService<T, U>>,
{
    type Request = http::Request<M::RequestBody>;
    type Response = http::Response<M::ResponseBody>;
    type Error = M::Error;
    type Service = WebService<T, U, M>;
    type InitError = Never;
    type Future = FutureResult<Self::Service, Self::InitError>;

    fn new_service(&self) -> Self::Future {
        let service = self.middleware.wrap(self.service.clone());

        future::ok(WebService::new(service))
    }
}
