use error;
use response::{Context, Serializer};
use util::BufStream;

use bytes::Buf;
use http;

/// Types that can be returned from resources as responses to HTTP requests.
///
/// Implementations of `Response` are responsible for encoding the value using
/// the appropriate content type. The content type may be specific to the type
/// in question, for example `serde_json::Value` implies a content type of
/// `application/json`.
///
/// Alternatively, the provided `context` may be used to encode the response in
/// most suitable content-type based on the request context. The content-type is
/// picked using the following factors:
///
/// * The HTTP request's `Accept` header value (not yet implemented).
/// * Any content type specified by the resource using annotations.
/// * Serialization formats that the application made available to the resource.
///
/// Implementations of `Response` are able to asynchronously stream the response
/// body if needed. This is done by setting the HTTP response body to a
/// `BufStream` type that supports streaming.
pub trait Response {
    /// Data chunk type.
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
    type Body = error::Map<T>;

    fn into_http<S: Serializer>(self, _: &Context<S>) -> http::Response<Self::Body> {
        self.map(error::Map::new)
    }
}
