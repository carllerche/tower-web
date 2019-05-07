use crate::config::Config;
use crate::error::{self, Error, Catch};
use http::status::StatusCode;
use crate::routing::{Resource, ResourceFuture, RouteSet, RouteMatch};
use crate::util::http::HttpFuture;
use crate::util::tuple::Either2;

use futures::{Future, Poll};
use http;
use tower_service::Service;

use std::fmt;
use std::sync::Arc;

/// Web service
pub struct RoutedService<T, U>
where
    T: Resource,
{
    /// Resource that handles the request
    resource: T,

    /// Error handler
    catch: U,

    /// Config
    config: Config,

    /// Route set. Processes request to determine how the resource will process
    /// it.
    routes: Arc<RouteSet<T::Destination>>,
}

/// Response future returned by `RoutedService`
#[derive(Debug)]
pub struct RoutedResponse<T, U>
where U: Catch,
{
    request: http::Request<()>,
    catch: U,
    state: State<T, U::Future>,
}

#[derive(Debug)]
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
            config: self.config.clone(),
            routes: self.routes.clone(),
        }
    }
}

impl<T, U> fmt::Debug for RoutedService<T, U>
where
    T: Resource + fmt::Debug,
    T::Destination: fmt::Debug,
    U: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("RoutedService")
            .field("resource", &self.resource)
            .field("catch", &self.catch)
            .field("routes", &self.routes)
            .finish()
    }
}

// ===== impl RoutedService =====

impl<T, U> RoutedService<T, U>
where T: Resource,
{
    /// Create a new `RoutedService`
    pub(crate) fn new(resource: T, catch: U, config: Config, routes: RouteSet<T::Destination>) -> Self {
        let routes = Arc::new(routes);

        RoutedService {
            resource,
            catch,
            config,
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
                let route_match = RouteMatch::new(&request, captures, &self.config);

                // Dispatch the requeest
                let pending = self.resource
                    .dispatch(destination, &route_match, body);

                State::Pending(pending)
            }
            None => {
                let error = Error::from(StatusCode::NOT_FOUND);
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
where T: ResourceFuture,
      U: Catch,
{
    type Item = http::Response<Either2<T::Body, error::Map<U::Body>>>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::State::*;
        use crate::util::tuple::Either2::*;
        use futures::Async::*;

        loop {
            let catching = match self.state {
                Pending(ref mut fut) => {
                    let error = match fut.poll_response(&self.request) {
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
                    let resp = try_ready!(HttpFuture::poll_http(fut))
                        .map(|body| B(error::Map::new(body)));

                    return Ok(Ready(resp));
                }
            };

            self.state = Catching(catching);
        }
    }
}
