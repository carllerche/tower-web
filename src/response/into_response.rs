use response::{Context, Serializer, MapErr};
use util::BufStream;

use bytes::{Buf, Bytes};
use http::{self, header};
use serde;

/// Types can be returned as responses to HTTP requests.
// TODO: Rename `Response` w/ an `into_http` fn
pub trait IntoResponse {
    /// Data chunk type
    type Buf: Buf;

    /// The HTTP response body type.
    type Body: BufStream<Item = Self::Buf, Error = ::Error>;

    /// Convert the value into a response future
    fn into_response<S: Serializer>(self, context: &Context<S>) -> http::Response<Self::Body>;
}

impl<T> IntoResponse for T
where
    T: serde::Serialize,
{
    type Buf = <Self::Body as BufStream>::Item;
    type Body = MapErr<Bytes>;

    fn into_response<S>(self, context: &Context<S>) -> http::Response<Self::Body>
    where
        S: Serializer,
    {
        // TODO: Improve and handle errors
        let body = MapErr::new(context.serialize(&self).unwrap());

        let mut response = http::Response::builder()
            // Customize response
            .status(200)
            .body(body)
            .unwrap();

        response
            .headers_mut()
            .entry(header::CONTENT_TYPE)
            .unwrap()
            .or_insert_with(|| context.content_type());

        response
    }
}
