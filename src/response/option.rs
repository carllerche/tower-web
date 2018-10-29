use super::{Context, Response, Serializer};
use error::ErrorKind;

use http;

impl<T: Response> Response for Option<T> {
    type Buf = T::Buf;
    type Body = T::Body;

    fn into_http<S: Serializer>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, ::Error> {
        match self {
            Some(inner) => Response::into_http(inner, context),
            None => Err(ErrorKind::not_found().into()),
        }
    }
}
