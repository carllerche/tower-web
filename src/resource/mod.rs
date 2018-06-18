pub mod tuple;

use Payload;
use response::IntoResponse;
use routing::{RouteMatch, RouteSet};

use bytes::Buf;
use futures::{Future, Stream};
use http;

/// A resource
///
/// TODO: Should `Send` be hard codeed?
pub trait Resource: Clone + Send + 'static {
    /// Identifies a route.
    type Destination: Clone + Send + Sync + 'static;

    /// Buffer yielded by the body. Represents a chunk of the body.
    type Buf: Buf;

    /// The HTTP response body type.
    type Body: Stream<Item = Self::Buf, Error = ::Error> + Send + 'static;

    /// Responses returned by the resource
    type Response: IntoResponse<Buf = Self::Buf, Body = Self::Body>;

    /// Response future
    type Future: Future<Item = Self::Response, Error = ::Error> + Send + 'static;

    /// Return the routes associated with the resource.
    fn routes(&self) -> RouteSet<Self::Destination>;

    fn dispatch<T: Payload>(
        &mut self,
        destination: Self::Destination,
        route_match: &RouteMatch,
        request: &http::Request<()>,
        payload: T,
    ) -> Self::Future;
}

/// Combine two resources
pub trait Chain<U> {
    type Resource;

    fn chain(self, other: U) -> Self::Resource;
}
