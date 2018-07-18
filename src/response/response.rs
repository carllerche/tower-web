use response::{Context, Serializer, MapErr};
use util::BufStream;

use bytes::Buf;
use http::{self, header};

use std::io;

/// Types can be returned as responses to HTTP requests.
pub trait Response {
    /// Data chunk type
    type Buf: Buf;

    /// The HTTP response body type.
    type Body: BufStream<Item = Self::Buf, Error = ::Error>;

    /// Convert the value into a response future
    fn into_http<S: Serializer>(self, context: &Context<S>) -> http::Response<Self::Body>;
}

impl Response for String {
    type Buf = io::Cursor<Vec<u8>>;
    type Body = MapErr<String>;

    fn into_http<S: Serializer>(self, context: &Context<S>) -> http::Response<Self::Body> {
        http::Response::builder()
            // Customize response
            .status(200)
            .header(header::CONTENT_TYPE, context.content_type())
            .body(MapErr::new(self))
            .unwrap()
    }
}

impl Response for &'static str {
    type Buf = io::Cursor<&'static [u8]>;
    type Body = MapErr<&'static str>;

    fn into_http<S: Serializer>(self, context: &Context<S>) -> http::Response<Self::Body> {
        http::Response::builder()
            // Customize response
            .status(200)
            .header(header::CONTENT_TYPE, context.content_type())
            .body(MapErr::new(self))
            .unwrap()
    }
}

impl<T> Response for http::Response<T>
where T: BufStream,
{
    type Buf = T::Item;
    type Body = MapErr<T>;

    fn into_http<S: Serializer>(self, _: &Context<S>) -> http::Response<Self::Body> {
        self.map(MapErr::new)
    }
}
