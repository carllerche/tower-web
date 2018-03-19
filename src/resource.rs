use routing::{self, RouteSet, Route, Destination, Match};

use bytes::Bytes;
use futures::{Future, Stream};
use futures::future::FutureResult;
use futures::stream::Once;
use http;

/// A resource
pub trait Resource: Clone + Send + 'static {
    /// The HTTP response body type.
    type Body: Stream<Item = Bytes, Error = ::Error> + Send + 'static;

    /// Response future
    type Future: Future<Item = http::Response<Self::Body>, Error = ::Error> + Send + 'static;

    /// Return the routes associated with the resource.
    fn routes(&self) -> RouteSet;

    fn dispatch(&mut self, route: &Match, request: http::Request<()>) -> Self::Future;
}

/// Combine two resources
pub trait Chain<U> {
    type Resource;

    fn chain(self, other: U) -> Self::Resource;
}

const ROUTE_MASK: usize = (1 << 16) - 1;

// ===== impl (...) =====

impl Resource for () {
    type Body = Once<Bytes, ::Error>;
    type Future = FutureResult<http::Response<Self::Body>, ::Error>;

    fn routes(&self) -> RouteSet {
        RouteSet::new()
    }

    fn dispatch(&mut self, route: &Match, request: http::Request<()>) -> Self::Future {
        unreachable!();
    }
}

impl<U> Chain<U> for () {
    type Resource = U;

    fn chain(self, other: U) -> Self::Resource {
        other
    }
}

impl<R1> Resource for (R1,)
where R1: Resource,
{
    type Body = R1::Body;
    type Future = R1::Future;

    fn routes(&self) -> RouteSet {
        self.0.routes()
    }

    fn dispatch(&mut self, rule: &Match, request: http::Request<()>) -> Self::Future {
        self.0.dispatch(rule, request)
    }
}

impl<R1, U> Chain<U> for (R1,) {
    type Resource = (R1, U);

    fn chain(self, other: U) -> Self::Resource {
        (self.0, other)
    }
}

impl<R1, R2> Resource for (R1, R2)
where R1: Resource,
      R2: Resource<Body = R1::Body>,
{
    type Body = R1::Body;
    type Future = ::futures::future::Either<R1::Future, R2::Future>;

    fn routes(&self) -> RouteSet {
        let mut routes = RouteSet::new();

        for route in self.0.routes() {
            let (destination, condition) = route.into_parts();
            let id = destination.id();

            assert!(id <= ROUTE_MASK);

            let destination = Destination::new((0 << 16) | id);

            routes.push(Route::new(destination, condition));
        }

        for route in self.1.routes() {
            let (destination, condition) = route.into_parts();
            let id = destination.id();

            assert!(id <= ROUTE_MASK);

            let destination = Destination::new((1 << 16) | id);

            routes.push(Route::new(destination, condition));
        }

        routes
    }

    fn dispatch(&mut self, match_: &Match, request: http::Request<()>) -> Self::Future {
        use futures::future::Either;

        let (destination, condition) = match_.into_parts();
        let id = destination.id();

        match id >> 16 {
            0 => {
                let d = Destination::new(id & ROUTE_MASK);
                let match_ = Match::new(&d, condition);
                Either::A(self.0.dispatch(&match_, request))
            }
            1 => {
                let d = Destination::new(id & ROUTE_MASK);
                let match_ = Match::new(&d, condition);
                Either::B(self.1.dispatch(&match_, request))
            }
            _ => unreachable!(),
        }
    }
}

impl<R1, R2, U> Chain<U> for (R1, R2) {
    type Resource = (R1, R2, U);

    fn chain(self, other: U) -> Self::Resource {
        (self.0, self.1, other)
    }
}
