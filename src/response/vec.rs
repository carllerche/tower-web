use crate::response::{Response, Serializer, Context, SerdeResponse};

use http;
use serde;

impl<T> Response for Vec<T>
where
    T: serde::Serialize,
{
    type Buf = <SerdeResponse<Self> as Response>::Buf;
    type Body = <SerdeResponse<Self> as Response>::Body;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where
        S: Serializer,
    {
        Response::into_http(SerdeResponse::new(self), context)
    }
}
