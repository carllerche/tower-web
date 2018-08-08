use error;
use response::Serializer;
use routing::{RouteMatch, RouteSet};
use util::BufStream;

use bytes::Buf;
use futures::Future;
use futures::future::FutureResult;
use http;

use std::marker::PhantomData;

/// A `Resource` processes HTTP requests received by the service.
///
/// A single service is composed of one or more resources. A resource instance
/// is created with a route set representing the set of routes that it is able
/// to handle. The service merges the route sets together and uses it to match
/// inbound requests.
///
/// When matching a route, the route set returns a `Self::Destination` value.
/// This value is then provided to `Resource::dispatch` and this is how the
/// resource instance knows which method to dispatch the request to.
///
/// Users are not expected to implement `Resource` themselves. Instead, the
/// `impl_web!` macro will generate a `Resource` implementation. The
/// implementation is then passed to `ServiceBuilder` to define the web service.
pub trait Resource: Clone {
    /// Token mapping a route to a resource method.
    type Destination: Clone + Send + Sync + 'static;

    /// The HTTP request body type.
    type RequestBody: BufStream;

    /// Buffer type yielded by the response body.
    type Buf: Buf;

    /// The HTTP response body type.
    ///
    /// This value will yield one or more `Self::Buf` values.
    type Body: BufStream<Item = Self::Buf, Error = ::Error>;

    /// Response future
    type Future: Future<Item = http::Response<Self::Body>, Error = ::Error>;

    /// Process the HTTP request and return the response asynchronously.
    ///
    /// The HTTP request has already been matched against the route set before
    /// calling this function. The `destination` and `route_match` arguments
    /// provide the necessary context for the resource to process the matched
    /// HTTP request.
    fn dispatch(
        &mut self,
        destination: Self::Destination,
        route_match: &RouteMatch,
        body: Self::RequestBody,
    ) -> Self::Future;
}

/// Convert a value into a `Resource`
///
///
pub trait IntoResource<S, RequestBody>
where S: Serializer,
      RequestBody: BufStream,
{
    /// Token mapping a route to a resource method.
    ///
    /// This will always be set to the same type as
    /// `Self::Resource::Destination`.
    type Destination: Clone + Send + Sync + 'static;

    /// The `Resource` value being converted to
    type Resource: Resource<Destination = Self::Destination,
                            RequestBody = RequestBody>;

    /// Returns the resource's set of routes.
    fn routes(&self) -> RouteSet<Self::Destination>;

    /// Convert the value into a resource
    fn into_resource(self, serializer: S) -> Self::Resource;
}

/// A resource with no methods.
///
/// Attempting to route to this resource will result in a 404 response.
pub struct Unit<B> {
    _p: PhantomData<B>,
}

impl<B> Unit<B> {
    /// Create a new `Unit` instance.
    pub fn new() -> Self {
        Unit { _p: PhantomData }
    }
}

impl<B> Resource for Unit<B>
where B: BufStream,
{
    type Destination = ();
    type RequestBody = B;
    type Buf = <Self::Body as BufStream>::Item;
    type Body = error::Map<String>;
    type Future = FutureResult<http::Response<Self::Body>, ::Error>;

    fn dispatch(&mut self, _: (), _: &RouteMatch, _: Self::RequestBody) -> Self::Future {
        unreachable!();
    }
}

impl<B> Clone for Unit<B> {
    fn clone(&self) -> Self {
        Unit::new()
    }
}
