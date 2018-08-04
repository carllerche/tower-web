use super::{Context, MapErr, Response, Serializer};

use http;
use http::header::{self, HeaderValue};

use std::io;

impl Response for String {
    type Buf = io::Cursor<Vec<u8>>;
    type Body = MapErr<String>;

    fn into_http<S: Serializer>(self, context: &Context<S>) -> http::Response<Self::Body> {
        respond(self, context)
    }
}

impl Response for &'static str {
    type Buf = io::Cursor<&'static [u8]>;
    type Body = MapErr<&'static str>;

    fn into_http<S: Serializer>(self, context: &Context<S>) -> http::Response<Self::Body> {
        respond(self, context)
    }
}

fn respond<T, S: Serializer>(value: T, context: &Context<S>) -> http::Response<MapErr<T>> {
    let content_type = context.content_type_header()
        .map(|content_type| content_type.clone())
        .unwrap_or_else(|| HeaderValue::from_static("text/plain"));

    http::Response::builder()
        // Customize response
        .status(200)
        .header(header::CONTENT_TYPE, content_type)
        .body(MapErr::new(value))
        .unwrap()
}
