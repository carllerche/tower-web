use response::{Context, Serializer};

use bytes::{Buf, Bytes};
use futures::stream::{self, Once, Stream};
use http::{self, header};
use serde;

use std::io::Cursor;

/// Types can be returned as responses to HTTP requests.
pub trait IntoResponse {
    /// Data chunk type
    type Buf: Buf;

    /// The HTTP response body type.
    type Body: Stream<Item = Self::Buf, Error = ::Error>;

    /// Convert the value into a response future
    fn into_response<S: Serializer>(self, context: &Context<S>)
        -> http::Response<Self::Body>;
}

impl<T> IntoResponse for T
where
    T: serde::Serialize,
{
    type Buf = Cursor<Bytes>;
    type Body = Once<Self::Buf, ::Error>;

    fn into_response<S>(self, context: &Context<S>) -> http::Response<Self::Body>
    where S: Serializer,
    {
        // TODO: Improve and handle errors
        let body = context.serialize(&self).unwrap();
        let body = Cursor::new(Bytes::from(body));

        let mut response = http::Response::builder()
            // Customize response
            .status(200)
            .body(stream::once(Ok(body)))
            .unwrap();

        response.headers_mut().entry(header::CONTENT_TYPE)
            .unwrap()
            .or_insert_with(|| context.content_type());

        response
    }
}
