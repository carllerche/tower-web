use futures::{Future, Poll};
use http;

/// HTTP response future trait
pub trait HttpFuture: sealed::Sealed {
    type Body;

    fn poll(&mut self) -> Poll<http::Response<Self::Body>, ::Error>;

    fn lift(self) -> LiftFuture<Self>
    where Self: Sized,
    {
        LiftFuture { inner: self }
    }
}

pub struct LiftFuture<T> {
    inner: T,
}

impl<T, B> HttpFuture for T
where T: Future<Item = http::Response<B>, Error = ::Error>
{
    type Body = B;

    fn poll(&mut self) -> Poll<http::Response<Self::Body>, ::Error> {
        Future::poll(self)
    }
}

impl<T, B> sealed::Sealed for T
where T: Future<Item = http::Response<B>, Error = ::Error>
{
}

impl<T: HttpFuture> Future for LiftFuture<T> {
    type Item = http::Response<T::Body>;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Self::Item, ::Error> {
        self.inner.poll()
    }
}

/// Must be made crate public for `Either{N}` implementations.
pub(crate) mod sealed {
    pub trait Sealed {}
}
