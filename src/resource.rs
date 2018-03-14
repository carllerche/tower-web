use bytes::Bytes;
use futures::{Future, Stream};
use futures::future::FutureResult;
use futures::stream::Once;
use http;

/// A resource
pub trait Resource: Clone + Send + 'static {
    /// The HTTP response body type.
    type Body: Stream<Item = Bytes, Error = Self::Error> + Send + 'static;

    /// The error type.
    type Error;

    /// Response future
    type Future: Future<Item = http::Response<Self::Body>, Error = Self::Error> + Send + 'static;

    fn call(&mut self) -> Self::Future;
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
    type Body = Once<Bytes, Self::Error>;
    type Error = ();
    type Future = FutureResult<http::Response<Self::Body>, Self::Error>;

    fn call(&mut self) -> Self::Future {
        unimplemented!();
    }
}
