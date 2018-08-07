use super::{Context, Response, Serializer};
use error;

use bytes::BytesMut;
use http::{self, header};
use tokio_fs::File;

use std::io;

const OCTET_STREAM: &'static str = "application/octet-stream";

impl Response for File {
    type Buf = io::Cursor<BytesMut>;
    type Body = error::Map<Self>;

    fn into_http<S: Serializer>(self, context: &Context<S>) -> http::Response<Self::Body> {
        let content_type = context.content_type_header()
            .map(|header| header.clone())
            .unwrap_or_else(|| header::HeaderValue::from_static(OCTET_STREAM));

        http::Response::builder()
            .status(200)
            .header(header::CONTENT_TYPE, content_type)
            .body(error::Map::new(self))
            .unwrap()
    }
}
