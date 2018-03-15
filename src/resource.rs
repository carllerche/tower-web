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

    fn call(&mut self, request: http::Request<()>) -> Self::Future;
}

/// Resource that matches all requests and returns 404.
#[derive(Clone)]
pub struct NotFound(());

impl NotFound {
    pub fn new() -> NotFound {
        NotFound(())
    }
}

impl Resource for NotFound {
    type Body = Once<Bytes, ::Error>;
    type Future = FutureResult<http::Response<Self::Body>, ::Error>;

    fn call(&mut self, request: http::Request<()>) -> Self::Future {
        unimplemented!();
    }
}
