use middleware::Middleware;
use util::buf_stream::BufStream;
use util::http::HttpService;

use http::{Request, Response};
use tower_service::Service;

/// An HTTP middleware
pub trait HttpMiddleware<S: HttpService>: sealed::Sealed<S> {
    type RequestBody: BufStream;
    type ResponseBody: BufStream;
    type Error;
    type Service: HttpService<RequestBody = Self::RequestBody,
                             ResponseBody = Self::ResponseBody,
                                    Error = Self::Error>;

    fn wrap(&self, inner: S) -> Self::Service;
}

impl<T, S, B1, B2, B3, B4> HttpMiddleware<S> for T
where T: Middleware<S, Request = Request<B1>,
                      Response = Response<B2>>,
      S: Service<Request = Request<B3>,
                Response = Response<B4>>,
      B1: BufStream,
      B2: BufStream,
      B3: BufStream,
      B4: BufStream,
{
    type RequestBody = B1;
    type ResponseBody = B2;
    type Error = T::Error;
    type Service = T::Service;

    fn wrap(&self, inner: S) -> Self::Service {
        Middleware::wrap(self, inner)
    }
}

impl<T, S, B1, B2, B3, B4> sealed::Sealed<S> for T
where T: Middleware<S, Request = Request<B3>,
                      Response = Response<B4>>,
      S: Service<Request = Request<B1>,
                Response = Response<B2>>,
      B1: BufStream,
      B2: BufStream,
      B3: BufStream,
      B4: BufStream,
{}

mod sealed {
    pub trait Sealed<S> {}
}
