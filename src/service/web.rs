use routing::{RouteSet, RouteMatch};
use service::Resource;
use util::BufStream;

use futures::{Future, Poll};
use http;
use tower_service::Service;

use std::marker::PhantomData;
use std::sync::Arc;

/// Web service
#[derive(Debug)]
pub struct WebService<T, ReqBody>
where
    T: Resource,
{
    /// Resource that handles the request
    resource: T,

    /// Route set. Processes request to determine how the resource will process
    /// it.
    routes: Arc<RouteSet<T::Destination>>,

    /// The request body type.
    _p: PhantomData<ReqBody>,
}

impl<T, ReqBody> Clone for WebService<T, ReqBody>
where
    T: Resource + Clone,
    ReqBody: BufStream,
{
    fn clone(&self) -> WebService<T, ReqBody> {
        WebService {
            resource: self.resource.clone(),
            routes: self.routes.clone(),
            _p: PhantomData,
        }
    }
}

// ===== impl WebService =====

impl<T, ReqBody> WebService<T, ReqBody>
where
    T: Resource,
    ReqBody: BufStream,
{
    pub(crate) fn new(resource: T, routes: Arc<RouteSet<T::Destination>>) -> Self {
        WebService {
            resource,
            routes,
            _p: PhantomData,
        }
    }
}

impl<T, ReqBody> Service for WebService<T, ReqBody>
where
    T: Resource<RequestBody = ReqBody>,
    ReqBody: BufStream,
{
    type Request = http::Request<ReqBody>;
    type Response = <Self::Future as Future>::Item;
    type Error = <Self::Future as Future>::Error;
    type Future = T::Future;
    // type Future = ResponseFuture<T, S>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        // Always ready
        Ok(().into())
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        // TODO: Use the body
        let (head, body) = request.into_parts();
        let request = http::Request::from_parts(head, ());

        match self.routes.test(&request) {
            Some((destination, params)) => {
                // Create the `RouteMatch` for the routing result
                let route_match = RouteMatch::new(request, params);

                // Dispatch the requeest
                self.resource
                    .dispatch(destination, route_match, body)
            }
            None => {
                unimplemented!("No route matches {:?}", request);
            }
        }
    }
}
