use bytes::Bytes;
use futures::{IntoFuture, Future, Stream, Poll};
use futures::stream::{self, Once};
use http;
use serde;

use std::fmt;

/// Convert a value into an HTTP response.
pub trait IntoResponse {
    /// The HTTP response body type.
    type Body: Stream<Item = Bytes, Error = ::Error>;

    /// Future of the response value
    type Future: Future<Item = http::Response<Self::Body>, Error = ::Error>;

    /// Convert the value into a response future
    fn into_response(self) -> Self::Future;
}

/// Do not serialize the value
pub struct Raw<T>(T);

/// Map a serializable response to an HTTP response
pub struct Serialize<T>(T);

// ===== impl IntoResponse =====

impl<T> IntoResponse for T
where T: IntoFuture,
      T::Item: serde::Serialize,
      T::Error: fmt::Debug,
{
    type Body = Once<Bytes, ::Error>;
    type Future = Serialize<T::Future>;

    fn into_response(self) -> Self::Future {
        Serialize(self.into_future())
    }
}

// ===== impl Serialize =====

impl<T> Future for Serialize<T>
where T: Future,
      T::Item: serde::Serialize,
      T::Error: fmt::Debug,
{
    type Item = http::Response<Once<Bytes, ::Error>>;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use futures::Async::*;

        let item = match self.0.poll() {
            Ok(Ready(v)) => v,
            Ok(NotReady) => return Ok(NotReady),
            Err(error) => {
                warn!("failed to process request; error = {:?}", error);

                let response = http::Response::builder()
                    .status(500)
                    .header("content-type", "text/plain")
                    .body(stream::once(Ok(Bytes::from_static(b"internal server error"))))
                    .unwrap();

                return Ok(response.into());
            }
        };

        let body = ::serde_json::to_vec(&item)
            .unwrap()
            .into();

        let response = http::Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body(stream::once(Ok(body)))
            .unwrap();

        Ok(response.into())
    }
}
