use bytes::Bytes;
use futures::{IntoFuture, Future, Stream, Poll};
use futures::stream::{self, Once};
use http;
use serde;

/// Convert a value into an HTTP response.
pub trait IntoResponse {
    /// The HTTP response body type.
    type Body: Stream<Item = Bytes, Error = Self::Error>;

    /// The error type.
    ///
    /// TODO: Should this always be an `http::Response<()>`?
    type Error;

    /// Future of the response value
    type Future: Future<Item = http::Response<Self::Body>, Error = Self::Error>;

    /// Convert the value into a response future
    fn into_response(self) -> Self::Future;
}

/// Do not serialize the value
pub struct Raw<T>(T);

/// Map a serializable response to an HTTP response
///
/// TODO: Rename?
pub struct Serialize<T>(T);

// impl<E> IntoResponse for Result<

impl<T> IntoResponse for T
where T: IntoFuture,
      T::Item: serde::Serialize,
{
    type Body = Once<Bytes, Self::Error>;
    type Error = T::Error;
    type Future = Serialize<T::Future>;

    fn into_response(self) -> Self::Future {
        Serialize(self.into_future())
    }
}

impl<T> Future for Serialize<T>
where T: Future,
      T::Item: serde::Serialize,
{
    type Item = http::Response<Once<Bytes, Self::Error>>;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let item = try_ready!(self.0.poll());
        let body = ::serde_json::to_vec(&item)
            .unwrap()
            .into();

        let resp = http::Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body(stream::once(Ok(body)))
            .unwrap();

        Ok(resp.into())
    }
}
