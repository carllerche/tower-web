use service::Payload;

use tower_service::Service;

use bytes::Buf;
use futures::Stream;
use http::{Request, Response};
use futures::{Future, Poll};

/// An HTTP service
///
/// This is not intended to be implemented directly. Instead, it is a trait
/// alias of sorts. Implements the `tower_service::Service` trait using
/// `http::Request` and `http::Response` types.
pub trait HttpService: ::util::sealed::Sealed {
    /// Request payload.
    type RequestBody: Payload;

    /// Buffer yielded by the reesposne body. Represents a chunk of the body.
    type ResponseBuf: Buf;

    /// The HTTP response body type.
    type ResponseBody: Stream<Item = Self::ResponseBuf, Error = ::Error>;

    /// The future response value.
    type Future: Future<Item = Response<Self::ResponseBody>, Error = ::Error>;

    /// Returns `Ready` when the service is able to process requests.
    fn poll_ready(&mut self) -> Poll<(), ::Error>;

    /// Process the request and return the response asynchronously.
    fn call(&mut self, request: Request<Self::RequestBody>) -> Self::Future;
}

impl<T, B1, B2> HttpService for T
where T: Service<Request = Request<B1>,
                Response = Response<B2>,
                   Error = ::Error>,
      B1: Payload,
      B2: Stream<Error = ::Error>,
      B2::Item: Buf,
{
    type RequestBody = B1;
    type ResponseBuf = B2::Item;
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
where T: Service<Request = Request<B1>,
                Response = Response<B2>>,
      B1: Payload,
      B2: Stream<Error = ::Error>,
      B2::Item: Buf,
{}
