use response::Serialize;

use bytes::Bytes;
use futures::stream::Once;
use futures::{Future, IntoFuture, Stream};
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

impl<T> IntoResponse for T
where
    T: IntoFuture,
    T::Item: serde::Serialize,
    T::Error: fmt::Debug,
{
    type Body = Once<Bytes, ::Error>;
    type Future = Serialize<T::Future>;

    fn into_response(self) -> Self::Future {
        Serialize::new(self.into_future())
    }
}
