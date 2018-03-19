use routing::{self, RouteSet, Match};

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

/// Resource that matches all requests and returns 404.
#[derive(Clone)]
pub struct NotFound(());

// ===== impl NotFound =====

impl NotFound {
    pub fn new() -> NotFound {
        NotFound(())
    }
}

impl Resource for NotFound {
    type Body = Once<Bytes, ::Error>;
    type Future = FutureResult<http::Response<Self::Body>, ::Error>;

    fn routes(&self) -> RouteSet {
        RouteSet::new()
    }

    fn dispatch(&mut self, route: &Match, request: http::Request<()>) -> Self::Future {
        unreachable!();
    }
}

// ===== impl (...) =====

impl<R1, R2> Resource for (R1, R2)
where R1: Resource,
      R2: Resource<Body = R1::Body>,
{
    type Body = R1::Body;
    type Future = ::futures::future::Either<R1::Future, R2::Future>;

    fn routes(&self) -> RouteSet {
        unimplemented!();
    }

    fn dispatch(&mut self, rute: &Match, request: http::Request<()>) -> Self::Future {
        unimplemented!();
    }
}
