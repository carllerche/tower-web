use crate::error;
use crate::response::{Response, Serializer, Context};
use crate::util::BufStream;

use bytes::Bytes;
use http::{self, header};
use serde;

/// Use a Serde value as an HTTP response
///
/// Takes a `T: serde::Serialize` and implements `Response` for it.
#[derive(Debug)]
pub struct SerdeResponse<T>(T);

impl<T> SerdeResponse<T> {
    /// Create a new `SerdeResponse` using the given value.
    pub fn new(value: T) -> SerdeResponse<T> {
        SerdeResponse(value)
    }
}

impl<T> Response for SerdeResponse<T>
where
    T: serde::Serialize,
{
    type Buf = <Self::Body as BufStream>::Item;
    type Body = error::Map<Bytes>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where
        S: Serializer,
    {
        let content_type = context.content_type_header()
            .expect("no content type specified for response");

        let serialize_context = context.serializer_context();

        let serialized = context.serialize(&self.0, &serialize_context)
            // TODO: Improve and handle errors
            .unwrap();

        let body = error::Map::new(serialized);

        let mut response = http::Response::builder()
            // Customize response
            .status(200)
            .body(body)
            .unwrap();

        response
            .headers_mut()
            .entry(header::CONTENT_TYPE)
            .unwrap()
            .or_insert_with(|| content_type.clone());

        Ok(response)
    }
}
