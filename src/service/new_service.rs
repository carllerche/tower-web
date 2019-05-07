use crate::error::Catch;
use crate::routing::{Resource, RoutedService};
use crate::service::WebService;
use crate::util::Never;
use crate::util::http::{HttpMiddleware};

use futures::future::{self, FutureResult};
use http;
use tower_service::NewService;

use std::fmt;

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
        let service = self.middleware.wrap_http(self.service.clone());

        future::ok(WebService::new(service))
    }
}

impl<T, U, M> fmt::Debug for NewWebService<T, U, M>
where
    T: Resource + fmt::Debug,
    T::Destination: fmt::Debug,
    U: fmt::Debug,
    M: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("NewService")
            .field("service", &self.service)
            .field("middleware", &self.middleware)
            .finish()
    }
}
