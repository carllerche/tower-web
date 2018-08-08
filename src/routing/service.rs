use error::{self, Error, ErrorKind, Catch};
use routing::{Resource, RouteSet, RouteMatch};
use util::http::HttpFuture;
use util::tuple::Either2;

use futures::{Future, Poll};
use http;
use tower_service::Service;

use std::sync::Arc;

/// Web service
#[derive(Debug)]
pub struct RoutedService<T, U>
where
    T: Resource,
{
    /// Resource that handles the request
    resource: T,

    /// Error handler
    catch: U,

    /// Route set. Processes request to determine how the resource will process
    /// it.
    routes: Arc<RouteSet<T::Destination>>,
}

pub struct RoutedResponse<T, U>
where U: Catch,
{
    request: http::Request<()>,
    catch: U,
    state: State<T, U::Future>,
}

enum State<T, U> {
    Pending(T),
    Catching(U),
}

impl<T, U> Clone for RoutedService<T, U>
where
    T: Resource,
    U: Clone,
{
    fn clone(&self) -> RoutedService<T, U> {
        RoutedService {
            resource: self.resource.clone(),
            catch: self.catch.clone(),
            routes: self.routes.clone(),
        }
    }
}

// ===== impl RoutedService =====

impl<T, U> RoutedService<T, U>
where T: Resource,
{
    /// Create a new `RoutedService`
    pub(crate) fn new(resource: T, catch: U, routes: RouteSet<T::Destination>) -> Self {
        let routes = Arc::new(routes);

        RoutedService {
            resource,
            catch,
            routes,
        }
    }
}

impl<T, U> Service for RoutedService<T, U>
where T: Resource,
      U: Catch,
{
    type Request = http::Request<T::RequestBody>;
    type Response = <Self::Future as Future>::Item;
    type Error = Error;
    type Future = RoutedResponse<T::Future, U>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        // Always ready
        Ok(().into())
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        // TODO: Use the body
        let (head, body) = request.into_parts();
        let request = http::Request::from_parts(head, ());

        let state = match self.routes.test(&request) {
            Some((destination, captures)) => {
                // Create the `RouteMatch` for the routing result
                let route_match = RouteMatch::new(&request, captures);

                // Dispatch the requeest
                let pending = self.resource
                    .dispatch(destination, &route_match, body);

                State::Pending(pending)
            }
            None => {
                let error = ErrorKind::not_found().into();
                let catching = self.catch.catch(&request, error);

                State::Catching(catching)
            }
        };

        let catch = self.catch.clone();

        RoutedResponse {
            request,
            catch,
            state,
        }
    }
}

// ===== impl RoutedResponse =====

impl<T, U> Future for RoutedResponse<T, U>
where T: HttpFuture,
      U: Catch,
{
    type Item = http::Response<Either2<T::Body, error::Map<U::Body>>>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::State::*;
        use util::tuple::Either2::*;
        use futures::Async::*;

        loop {
            let catching = match self.state {
                Pending(ref mut fut) => {
                    let error = match fut.poll() {
                        Ok(Ready(v)) => {
                            let v = v.map(A);
                            return Ok(Ready(v))
                        }
                        Ok(NotReady) => return Ok(NotReady),
                        Err(e) => e,
                    };

                    self.catch.catch(&self.request, error)
                }
                Catching(ref mut fut) => {
                    let resp = try_ready!(HttpFuture::poll(fut))
                        .map(|body| B(error::Map::new(body)));

                    return Ok(Ready(resp));
                }
            };

            self.state = Catching(catching);
        }
    }
}
