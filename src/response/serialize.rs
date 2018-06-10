use bytes::Bytes;
use futures::stream::{self, Once};
use futures::{Future, Poll};
use http;
use serde;

use std::fmt;

/// Map a serializable response to an HTTP response
pub struct Serialize<T>(T);

impl<T> Serialize<T> {
    pub(crate) fn new(inner: T) -> Serialize<T> {
        Serialize(inner)
    }
}

impl<T> Future for Serialize<T>
where
    T: Future,
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
                    .body(stream::once(Ok(Bytes::from_static(
                        b"internal server error",
                    ))))
                    .unwrap();

                return Ok(response.into());
            }
        };

        let body = ::serde_json::to_vec(&item).unwrap().into();

        let response = http::Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body(stream::once(Ok(body)))
            .unwrap();

        Ok(response.into())
    }
}
