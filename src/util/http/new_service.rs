use super::HttpService;
use crate::util::buf_stream::BufStream;

use futures::Future;
use http::{Request, Response};
use tower_service::NewService;

/// Creates `HttpService` values.
///
/// This is not intended to be implemented directly. Instead, it is a trait
/// alias of sorts, aliasing `tower_service::NewService` trait with
/// `http::Request` and `http::Response` types.
pub trait NewHttpService: sealed::Sealed {
    /// The HTTP request body handled by the service.
    type RequestBody: BufStream;

    /// The HTTP response body returned by the service.
    type ResponseBody: BufStream;

    /// Errors produced by the service
    type Error;

    /// The `Service` value created by this factory
    type Service: HttpService<RequestBody = Self::RequestBody,
                             ResponseBody = Self::ResponseBody,
                                    Error = Self::Error>;

    /// Errors produced while building a service.
    type InitError;

    /// The future of the `Service` instance.
    type Future: Future<Item = Self::Service, Error = Self::InitError>;

    /// Create and return a new service value asynchronously.
    fn new_http_service(&self) -> Self::Future;
}

impl<T, B1, B2> NewHttpService for T
where T: NewService<Request = Request<B1>,
                   Response = Response<B2>>,
      B1: BufStream,
      B2: BufStream
{
    type RequestBody = B1;
    type ResponseBody = B2;
    type Error = T::Error;
    type Service = T::Service;
    type InitError = T::InitError;
    type Future = T::Future;

    fn new_http_service(&self) -> Self::Future {
        NewService::new_service(self)
    }
}

impl<T, B1, B2> sealed::Sealed for T
where T: NewService<Request = Request<B1>,
                   Response = Response<B2>>,
      B1: BufStream,
      B2: BufStream
{}

mod sealed {
    pub trait Sealed {}
}
