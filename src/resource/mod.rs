pub mod tuple;

use routing::{RouteSet, Match};

use bytes::Bytes;
use futures::{Future, Stream};
use http;

/// A resource
pub trait Resource: Clone + Send + 'static {
    /// Identifies a route.
    type Destination: Clone + Send + Sync + 'static;

    /// The HTTP response body type.
    type Body: Stream<Item = Bytes, Error = ::Error> + Send + 'static;

    /// Response future
    type Future: Future<Item = http::Response<Self::Body>, Error = ::Error> + Send + 'static;

    /// Return the routes associated with the resource.
    fn routes(&self) -> RouteSet<Self::Destination>;

    fn dispatch(&mut self,
                route: Match<Self::Destination>,
                request: http::Request<()>) -> Self::Future;
}

/// Combine two resources
pub trait Chain<U> {
    type Resource;

    fn chain(self, other: U) -> Self::Resource;
}
