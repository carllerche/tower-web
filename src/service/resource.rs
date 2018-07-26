use response::{Serializer, MapErr};
use routing::{RouteMatch, RouteSet};
use util::BufStream;

use bytes::Buf;
use futures::Future;
use futures::future::FutureResult;
use http;

use std::marker::PhantomData;

/// A resource
pub trait Resource: Clone {
    /// Identifies a route.
    type Destination: Clone + Send + Sync + 'static;

    type RequestBody: BufStream;

    /// Buffer yielded by the body. Represents a chunk of the body.
    /// TODO: Get rid of this?
    type Buf: Buf;

    /// The HTTP response body type.
    /// TODO: Rename this -> ResponseBody?
    type Body: BufStream<Item = Self::Buf, Error = ::Error>;

    /// Response future
    type Future: Future<Item = http::Response<Self::Body>, Error = ::Error>;

    fn dispatch(
        &mut self,
        destination: Self::Destination,
        route_match: RouteMatch,
        body: Self::RequestBody,
    ) -> Self::Future;
}

/// Convert a value into a `Resource`
pub trait IntoResource<S, RequestBody>
where S: Serializer,
      RequestBody: BufStream,
{
    /// Token used by a `Resource` to match a request with a handler.
    type Destination: Clone + Send + Sync + 'static;

    /// The `Resource` value being converted to
    type Resource: Resource<Destination = Self::Destination,
                            RequestBody = RequestBody>;

    /// Returns the resource's set of routes.
    fn routes(&self) -> RouteSet<Self::Destination>;

    /// Convert the value into a resource
    fn into_resource(self, serializer: S) -> Self::Resource;
}

pub struct Unit<B> {
    _p: PhantomData<B>,
}

impl<B> Unit<B> {
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
    type Body = MapErr<String>;
    type Future = FutureResult<http::Response<Self::Body>, ::Error>;

    fn dispatch(&mut self, _: (), _: RouteMatch, _: Self::RequestBody) -> Self::Future {
        unreachable!();
    }
}

impl<B> Clone for Unit<B> {
    fn clone(&self) -> Self {
        Unit::new()
    }
}
