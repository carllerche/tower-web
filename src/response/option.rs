use super::{Context, Response, Serializer};
use http::status::StatusCode;

use http;

impl<T: Response> Response for Option<T> {
    type Buf = T::Buf;
    type Body = T::Body;

    fn into_http<S: Serializer>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error> {
        match self {
            Some(inner) => Response::into_http(inner, context),
            None => Err(StatusCode::NOT_FOUND.into()),
        }
    }
}
