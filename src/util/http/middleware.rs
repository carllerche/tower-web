use crate::middleware::Middleware;
use crate::util::buf_stream::BufStream;
use crate::util::http::HttpService;

use http::{Request, Response};

/// HTTP middleware trait
///
/// A trait "alias" for `Middleware` where the yielded service is an
/// `HttpService`.
///
/// Using `HttpMiddleware` in where bounds is easier than trying to use `Middleware`
/// directly.
pub trait HttpMiddleware<S, RequestBody>: sealed::Sealed<S, RequestBody> {
    /// The HTTP response body returned by the wrapped service.
    type ResponseBody: BufStream;

    /// The wrapped service's error type.
    type Error;

    /// The wrapped service.
    type Service: HttpService<RequestBody,
                             ResponseBody = Self::ResponseBody,
                                    Error = Self::Error>;

    /// Wrap the given service with the middleware, returning a new servicee
    /// that has been decorated with the middleware.
    fn wrap_http(&self, inner: S) -> Self::Service;
}

impl<T, S, B1, B2> HttpMiddleware<S, B1> for T
where T: Middleware<S, Request<B1>, Response = Response<B2>>,
      B1: BufStream,
      B2: BufStream,
{
    type ResponseBody = B2;
    type Error = T::Error;
    type Service = T::Service;

    fn wrap_http(&self, inner: S) -> Self::Service {
        Middleware::wrap(self, inner)
    }
}

impl<T, S, B1, B2> sealed::Sealed<S, B1> for T
where T: Middleware<S, Request<B1>, Response = Response<B2>>,
      B1: BufStream,
      B2: BufStream,
{}

mod sealed {
    pub trait Sealed<T, S> {}
}
