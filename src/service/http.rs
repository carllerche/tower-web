use util::BufStream;

use tower_service::Service;

use futures::{Future, Poll};
use http::{Request, Response};

/// An HTTP service
///
/// This is not intended to be implemented directly. Instead, it is a trait
/// alias of sorts. Implements the `tower_service::Service` trait using
/// `http::Request` and `http::Response` types.
pub trait HttpService: ::util::sealed::Sealed {
    /// Request payload.
    type RequestBody: BufStream;

    /// The HTTP response body type.
    type ResponseBody: BufStream<Error = ::Error>;

    /// The future response value.
    type Future: Future<Item = Response<Self::ResponseBody>, Error = ::Error>;

    /// Returns `Ready` when the service is able to process requests.
    fn poll_ready(&mut self) -> Poll<(), ::Error>;

    /// Process the request and return the response asynchronously.
    fn call(&mut self, request: Request<Self::RequestBody>) -> Self::Future;
}

impl<T, B1, B2> HttpService for T
where
    T: Service<Request = Request<B1>, Response = Response<B2>, Error = ::Error>,
    B1: BufStream,
    B2: BufStream<Error = ::Error>,
{
    type RequestBody = B1;
    type ResponseBody = B2;
    type Future = T::Future;

    fn poll_ready(&mut self) -> Poll<(), ::Error> {
        Service::poll_ready(self)
    }

    fn call(&mut self, request: Request<Self::RequestBody>) -> Self::Future {
        Service::call(self, request)
    }
}

impl<T, B1, B2> ::util::sealed::Sealed for T
where
    T: Service<Request = Request<B1>, Response = Response<B2>>,
    B1: BufStream,
    B2: BufStream<Error = ::Error>,
{
}
