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
pub struct WebService<T, In>
where
    T: Resource,
{
    /// Resource that handles the request
    resource: T,

    /// Route set. Processes request to determine how the resource will process
    /// it.
    routes: Arc<RouteSet<T::Destination>>,

    /// The request body type.
    _p: PhantomData<In>,
}

impl<T: Resource + Clone, In> Clone for WebService<T, In>
where
    T: Resource + Clone,
{
    fn clone(&self) -> WebService<T, In> {
        WebService {
            resource: self.resource.clone(),
            routes: self.routes.clone(),
            _p: PhantomData,
        }
    }
}

// ===== impl WebService =====

impl<T, In> WebService<T, In>
where
    T: Resource,
{
    pub(crate) fn new(resource: T, routes: RouteSet<T::Destination>) -> Self {
        let routes = Arc::new(routes);

        WebService {
            resource,
            routes,
            _p: PhantomData,
        }
    }
}

impl<T, In> Service for WebService<T, In>
where
    T: Resource,
    In: BufStream,
{
    type Request = http::Request<In>;
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
                unimplemented!();
            }
        }
    }
}
