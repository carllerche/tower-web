use Error;
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
pub struct NewWebService<T, RequestBody>
where
    T: Resource,
{
    /// Resource that handles the request
    resource: T,

    /// Route set. Processes request to determine how the resource will process
    /// it.
    routes: Arc<RouteSet<T::Destination>>,

    /// The request body type.
    _p: PhantomData<RequestBody>,
}

impl<T, RequestBody> NewWebService<T, RequestBody>
where
    T: Resource,
    RequestBody: BufStream,
{
    /// Create a new `NewWebService` instance.
    pub(crate) fn new(resource: T, routes: RouteSet<T::Destination>) -> Self {
        let routes = Arc::new(routes);

        NewWebService {
            resource,
            routes,
            _p: PhantomData,
        }
    }
}


impl<T, RequestBody> NewService for NewWebService<T, RequestBody>
where
    T: Resource<RequestBody = RequestBody>,
    RequestBody: BufStream,
{
    type Request = http::Request<RequestBody>;
    type Response = <T::Future as Future>::Item;
    type Error = Error;
    type Service = WebService<T, RequestBody>;
    type InitError = Never;
    type Future = FutureResult<Self::Service, Self::InitError>;

    fn new_service(&self) -> Self::Future {
        let service = WebService::new(
            self.resource.clone(),
            self.routes.clone());

        future::ok(service)
    }
}
