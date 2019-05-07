use super::{Context, Response, Serializer};
use crate::error;

use http;
use http::header::{self, HeaderValue};

use std::io;

impl Response for String {
    type Buf = io::Cursor<Vec<u8>>;
    type Body = error::Map<String>;

    fn into_http<S: Serializer>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error> {
        respond(self, context)
    }
}

impl Response for &'static str {
    type Buf = io::Cursor<&'static [u8]>;
    type Body = error::Map<&'static str>;

    fn into_http<S: Serializer>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error> {
        respond(self, context)
    }
}

fn respond<T, S: Serializer>(value: T, context: &Context<S>)
    -> Result<http::Response<error::Map<T>>, crate::Error>
{
    let content_type = context.content_type_header()
        .map(|content_type| content_type.clone())
        .unwrap_or_else(|| HeaderValue::from_static("text/plain"));

    let response = http::Response::builder()
        // Customize response
        .status(200)
        .header(header::CONTENT_TYPE, content_type)
        .body(error::Map::new(value))
        .unwrap();

    Ok(response)
}
