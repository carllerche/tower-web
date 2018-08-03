use routing::{Resource, RoutedService};
use util::http::{HttpMiddleware, HttpService};

use futures::Poll;
use http;
use tower_service::Service;

use std::fmt;

/// Web service
pub struct WebService<T, M>
where
    T: Resource,
    M: HttpMiddleware<RoutedService<T>>,
{
    /// The routed service wrapped with middleware
    inner: M::Service,
}

impl<T, M> WebService<T, M>
where
    T: Resource,
    M: HttpMiddleware<RoutedService<T>>,
{
    pub(crate) fn new(inner: M::Service) -> WebService<T, M> {
        WebService { inner }
    }
}

impl<T, M> Service for WebService<T, M>
where
    T: Resource,
    M: HttpMiddleware<RoutedService<T>>,
{
    type Request = http::Request<M::RequestBody>;
    type Response = http::Response<M::ResponseBody>;
    type Error = M::Error;
    type Future = <M::Service as HttpService>::Future;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_ready()
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        self.inner.call(request)
    }
}

impl<T, M> fmt::Debug for WebService<T, M>
where T: Resource + fmt::Debug,
      M: HttpMiddleware<RoutedService<T>> + fmt::Debug,
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
