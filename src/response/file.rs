use super::{Context, MapErr, Response, Serializer};

use bytes::BytesMut;
use http::{self, header};
use tokio_fs::File;

use std::io;

impl Response for File {
    type Buf = io::Cursor<BytesMut>;
    type Body = MapErr<Self>;

    fn into_http<S: Serializer>(self, context: &Context<S>) -> http::Response<Self::Body> {
        http::Response::builder()
            .status(200)
            .header(header::CONTENT_TYPE, context.content_type())
            .body(MapErr::new(self))
            .unwrap()
    }
}
