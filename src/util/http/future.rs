use futures::{Future, Poll};
use http;

/// HTTP response future trait
///
/// A trait "alias" for `Future` where the yielded item is an `http::Response`.
///
/// Using `HttpFuture` in where bounds is easier than trying to use `Future`
/// directly.
pub trait HttpFuture: sealed::Sealed {
    /// The HTTP response body
    type Body;

    /// Attempt to resolve the future to a final value, registering the current
    /// task for wakeup if the value is not yet available.
    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error>;

    /// Wraps `self` with `LiftFuture`. This provides an implementation of
    /// `Future` for `Self`.
    fn lift(self) -> LiftFuture<Self>
    where Self: Sized,
    {
        LiftFuture { inner: self }
    }
}

/// Contains an `HttpFuture` providing an implementation of `Future`.
#[derive(Debug)]
pub struct LiftFuture<T> {
    inner: T,
}

impl<T, B> HttpFuture for T
where T: Future<Item = http::Response<B>, Error = crate::Error>
{
    type Body = B;

    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error> {
        Future::poll(self)
    }
}

impl<T, B> sealed::Sealed for T
where T: Future<Item = http::Response<B>, Error = crate::Error>
{
}

impl<T: HttpFuture> Future for LiftFuture<T> {
    type Item = http::Response<T::Body>;
    type Error = crate::Error;

    fn poll(&mut self) -> Poll<Self::Item, crate::Error> {
        self.inner.poll_http()
    }
}

/// Must be made crate public for `Either{N}` implementations.
pub(crate) mod sealed {
    pub trait Sealed {}
}
