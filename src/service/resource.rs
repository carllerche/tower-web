use response::{IntoResponse, Serializer};
use routing::{RouteMatch, RouteSet};
use util::BufStream;

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
    type Body: Stream<Item = Self::Buf, Error = ::Error>;

    /// Responses returned by the resource
    type Response: IntoResponse<Buf = Self::Buf, Body = Self::Body>;

    /// Response future
    type Future: Future<Item = Self::Response, Error = ::Error>;

    /// Return the routes associated with the resource.
    fn routes<S: Serializer>(&self, serializer: &S)
        -> RouteSet<Self::Destination, S::ContentType>;

    fn dispatch<In: BufStream>(
        &mut self,
        destination: Self::Destination,
        route_match: &RouteMatch,
        request: &http::Request<()>,
        payload: In,
    ) -> Self::Future;
}
