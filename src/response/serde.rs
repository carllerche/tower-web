use response::{Response, MapErr, Serializer, Context};
use util::BufStream;

use bytes::Bytes;
use http::{self, header};
use serde;

pub struct SerdeResponse<T>(T);

impl<T> SerdeResponse<T> {
    pub fn new(value: T) -> SerdeResponse<T> {
        SerdeResponse(value)
    }
}

impl<T> Response for SerdeResponse<T>
where
    T: serde::Serialize,
{
    type Buf = <Self::Body as BufStream>::Item;
    type Body = MapErr<Bytes>;

    fn into_http<S>(self, context: &Context<S>) -> http::Response<Self::Body>
    where
        S: Serializer,
    {
        // TODO: Improve and handle errors
        let body = MapErr::new(context.serialize(&self.0).unwrap());

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
