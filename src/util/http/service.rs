use crate::util::BufStream;

use futures::{Future, Poll};
use http::{Request, Response};
use tower_service::Service;

/// An HTTP service
///
/// This is not intended to be implemented directly. Instead, it is a trait
/// alias of sorts, aliasing `tower_service::Service` trait with `http::Request`
/// and `http::Response` types.
pub trait HttpService: sealed::Service {
    /// Request payload.
    type RequestBody: BufStream;

    /// The HTTP response body type.
    type ResponseBody: BufStream;

    /// The service error type
    type Error;

    /// The future response value.
    type Future: Future<Item = Response<Self::ResponseBody>, Error = Self::Error>;

    /// Returns `Ready` when the service is able to process requests.
    fn poll_http_ready(&mut self) -> Poll<(), Self::Error>;

    /// Process the request and return the response asynchronously.
    fn call_http(&mut self, request: Request<Self::RequestBody>) -> Self::Future;

    /// Wraps `self` with `LiftService`. This provides an implementation of
    /// `Service` for `Self`.
    fn lift(self) -> LiftService<Self>
    where Self: Sized,
    {
        LiftService { inner: self }
    }
}

/// Contains an `HttpService` providing an implementation of `Service`.
#[derive(Debug)]
pub struct LiftService<T> {
    inner: T,
}

impl<T, B1, B2> HttpService for T
where
    T: Service<Request = Request<B1>, Response = Response<B2>>,
    B1: BufStream,
    B2: BufStream,
{
    type RequestBody = B1;
    type ResponseBody = B2;
    type Error = T::Error;
    type Future = T::Future;

    fn poll_http_ready(&mut self) -> Poll<(), T::Error> {
        Service::poll_ready(self)
    }

    fn call_http(&mut self, request: Request<Self::RequestBody>) -> Self::Future {
        Service::call(self, request)
    }
}

impl<T> LiftService<T> {
    /// Return a reference to the underlying `HttpServce`.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Return a mutable reference to the underlying `HttpServce`.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Consumes `self`, returning the underlying `HttpServce`.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T> Service for LiftService<T>
where T: HttpService,
{
    type Request = Request<T::RequestBody>;
    type Response = Response<T::ResponseBody>;
    type Error = T::Error;
    type Future = T::Future;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_http_ready()
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        self.inner.call_http(request)
    }
}

impl<T, B1, B2> sealed::Service for T
where
    T: Service<Request = Request<B1>, Response = Response<B2>>,
    B1: BufStream,
    B2: BufStream
{
}

mod sealed {
    pub trait Service {}
}
