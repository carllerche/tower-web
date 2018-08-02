use routing::{RouteSet, Resource};
use service::{WebService, HttpMiddleware, HttpService, LiftService};
use util::{Never};

use futures::future::{self, FutureResult};
use http;
use tower_service::NewService;

use std::sync::Arc;

/// Creates new `WebService` values.
///
/// Instances of this type are created by `ServiceBuilder`. A `NewWebService`
/// instance is used to generate a `WebService` instance per connection.
#[derive(Debug)]
pub struct NewWebService<T, M>
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
}

impl<T, M> NewWebService<T, M>
where
    T: Resource,
    M: HttpMiddleware<WebService<T>>,
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
        }
    }
}

impl<T, M> NewService for NewWebService<T, M>
where
    T: Resource,
    M: HttpMiddleware<WebService<T>>,
{
    type Request = http::Request<M::RequestBody>;
    type Response = http::Response<M::ResponseBody>;
    type Error = M::Error;
    type Service = LiftService<M::Service>;
    type InitError = Never;
    type Future = FutureResult<Self::Service, Self::InitError>;

    fn new_service(&self) -> Self::Future {
        let service = self.middleware.wrap({
            WebService::new(
                self.resource.clone(),
                self.routes.clone())
        });

        future::ok(service.lift())
    }
}
