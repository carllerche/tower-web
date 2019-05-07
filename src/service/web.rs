use crate::error::Catch;
use crate::routing::{Resource, RoutedService};
use crate::util::http::{HttpMiddleware, HttpService};

use futures::Poll;
use http;
use tower_service::Service;

use std::fmt;

/// The service defined by `ServiceBuilder`.
///
/// `WebService` contains the resources, routes, middleware, catch handlers, ...
/// that were defined by the builder. It implements `tower_service::Service`,
/// which exposes an HTTP request / response API.
pub struct WebService<T, U, M>
where
    T: Resource,
    U: Catch,
    M: HttpMiddleware<RoutedService<T, U>>,
{
    /// The routed service wrapped with middleware
    inner: M::Service,
}

impl<T, U, M> WebService<T, U, M>
where
    T: Resource,
    U: Catch,
    M: HttpMiddleware<RoutedService<T, U>>,
{
    pub(crate) fn new(inner: M::Service) -> WebService<T, U, M> {
        WebService { inner }
    }
}

impl<T, U, M> Service for WebService<T, U, M>
where
    T: Resource,
    U: Catch,
    M: HttpMiddleware<RoutedService<T, U>>,
{
    type Request = http::Request<M::RequestBody>;
    type Response = http::Response<M::ResponseBody>;
    type Error = M::Error;
    type Future = <M::Service as HttpService>::Future;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_http_ready()
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        self.inner.call_http(request)
    }
}

impl<T, U, M> fmt::Debug for WebService<T, U, M>
where T: Resource + fmt::Debug,
      U: Catch + fmt::Debug,
      M: HttpMiddleware<RoutedService<T, U>> + fmt::Debug,
      M::Service: fmt::Debug,
      M::RequestBody: fmt::Debug,
      M::ResponseBody: fmt::Debug,
      M::Error: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("WebService")
            .field("inner", &self.inner)
            .finish()
    }
}
