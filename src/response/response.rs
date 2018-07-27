use response::{Context, Serializer, MapErr};
use util::BufStream;

use bytes::Buf;
use http;

/// Types can be returned as responses to HTTP requests.
pub trait Response {
    /// Data chunk type
    ///
    /// This type is always `Body::Buf`.
    type Buf: Buf;

    /// The HTTP response body type.
    type Body: BufStream<Item = Self::Buf, Error = ::Error>;

    /// Convert the value into a response future
    fn into_http<S: Serializer>(self, context: &Context<S>) -> http::Response<Self::Body>;
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
