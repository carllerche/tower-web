use crate::error;
use crate::response::{Context, Response, Serializer};
use crate::util::BufStream;

use bytes::Bytes;
use http;
use http::header::{self, HeaderValue};
use serde_json::{self, Value};

impl Response for Value {
    type Buf = <Self::Body as BufStream>::Item;
    type Body = error::Map<Bytes>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where
        S: Serializer,
    {
        // TODO: Improve error handling
        let body = serde_json::to_vec(&self).unwrap();

        // TODO: Improve and handle errors
        let body = error::Map::new(Bytes::from(body));

        let mut response = http::Response::builder()
            // Customize response
            .status(200)
            .body(body)
            .unwrap();

        response
            .headers_mut()
            .entry(header::CONTENT_TYPE)
            .unwrap()
            .or_insert_with(|| {
                context.content_type_header()
                    .map(|content_type| content_type.clone())
                    .unwrap_or_else(|| {
                        HeaderValue::from_static("application/json")
                    })
            });

        Ok(response)
    }
}
