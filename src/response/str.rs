use super::{Context, MapErr, Response, Serializer};

use http::{self, header};

use std::io;

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
