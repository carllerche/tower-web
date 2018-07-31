use Error;
use middleware::Middleware;
use routing::RouteSet;
use service::{Resource, WebService};
use util::{BufStream, Never};

use futures::Future;
use futures::future::{self, FutureResult};
use http;
use tower_service::NewService;

use std::marker::PhantomData;
use std::sync::Arc;

/// Creates new `WebService` values.
///
/// Instances of this type are created by `ServiceBuilder`. A `NewWebService`
/// instance is used to generate a `WebService` instance per connection.
#[derive(Debug)]
pub struct NewWebService<T, M, RequestBody>
where
    T: Resource,
{
    /// Resource that handles the request
    resource: T,

    /// Middleware to wrap the service with
    middleware: M,

    /// Route set. Processes request to determine how the resource will process
    /// it.
    routes: Arc<RouteSet<T::Destination>>,

    /// The request body type.
    _p: PhantomData<RequestBody>,
}

impl<T, M, RequestBody> NewWebService<T, M, RequestBody>
where
    T: Resource<RequestBody = RequestBody>,
    M: Middleware<WebService<T, RequestBody>>,
    RequestBody: BufStream,
{
    /// Create a new `NewWebService` instance.
    pub(crate) fn new(resource: T,
                      middleware: M,
                      routes: RouteSet<T::Destination>)
        -> Self
    {
        let routes = Arc::new(routes);

        NewWebService {
            resource,
            middleware,
            routes,
            _p: PhantomData,
        }
    }
}

impl<T, M, RequestBody, ResponseBody> NewService for NewWebService<T, M, RequestBody>
where
    T: Resource<RequestBody = RequestBody>,
    M: Middleware<WebService<T, RequestBody>, Request = http::Request<RequestBody>,
                                             Response = http::Response<ResponseBody>>,
    RequestBody: BufStream,
    ResponseBody: BufStream,
{
    type Request = http::Request<RequestBody>;
    type Response = M::Response;
    type Error = M::Error;
    type Service = M::Service;
    type InitError = Never;
    type Future = FutureResult<Self::Service, Self::InitError>;

    fn new_service(&self) -> Self::Future {
        let service = self.middleware.wrap({
            WebService::new(
                self.resource.clone(),
                self.routes.clone())
        });

        future::ok(service)
    }
}
