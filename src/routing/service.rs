use routing::{Resource, RouteSet, RouteMatch};

use futures::{Future, Poll};
use http;
use tower_service::Service;

use std::sync::Arc;

/// Web service
#[derive(Debug)]
pub struct RoutedService<T>
where
    T: Resource,
{
    /// Resource that handles the request
    resource: T,

    /// Route set. Processes request to determine how the resource will process
    /// it.
    routes: Arc<RouteSet<T::Destination>>,
}

impl<T> Clone for RoutedService<T>
where
    T: Resource + Clone,
{
    fn clone(&self) -> RoutedService<T> {
        RoutedService {
            resource: self.resource.clone(),
            routes: self.routes.clone(),
        }
    }
}

// ===== impl RoutedService =====

impl<T> RoutedService<T>
where T: Resource,
{
    /// Create a new `RoutedService`
    pub(crate) fn new(resource: T, routes: RouteSet<T::Destination>) -> Self {
        let routes = Arc::new(routes);

        RoutedService {
            resource,
            routes,
        }
    }
}

impl<T> Service for RoutedService<T>
where T: Resource,
{
    type Request = http::Request<T::RequestBody>;
    type Response = <Self::Future as Future>::Item;
    type Error = <Self::Future as Future>::Error;
    type Future = T::Future;

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
