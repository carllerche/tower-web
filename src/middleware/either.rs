use tokio::prelude::{Poll, future::Either};
use tower_service;
use futures::{Future, Async};
use http;

use super::Middleware;

impl<S, L, R, LB, RB> Middleware<S> for Either<L, R>
where
    L: Middleware<S, Response = http::Response<LB>>,
    R: Middleware<S, Response = http::Response<RB>, Request = L::Request, Error = L::Error>,
{
    type Request = L::Request;
    type Response = http::Response<Either<LB, RB>>;
    type Error = L::Error;
    type Service = EitherMiddlewareService<L::Service, R::Service>;

    fn wrap(&self, inner: S) -> Self::Service {
        match *self {
            Either::A(ref l) => EitherMiddlewareService(Either::A(l.wrap(inner))),
            Either::B(ref r) => EitherMiddlewareService(Either::B(r.wrap(inner))),
        }
    }
}

#[derive(Debug)]
pub struct EitherMiddlewareService<L, R>(Either<L, R>);

impl<L, R, LB, RB> tower_service::Service for EitherMiddlewareService<L, R>
where
    L: tower_service::Service<Response = http::Response<LB>>,
    R: tower_service::Service<Response = http::Response<RB>, Request = L::Request, Error = L::Error>,
{
    type Request = L::Request;
    type Response = http::Response<Either<LB, RB>>;
    type Error = L::Error;
    type Future = EitherMiddlewareFuture<L::Future, R::Future>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        match self.0 {
            Either::A(ref mut l) => l.poll_ready(),
            Either::B(ref mut r) => r.poll_ready(),
        }
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        EitherMiddlewareFuture(match self.0 {
            Either::A(ref mut l) => Either::A(l.call(req)),
            Either::B(ref mut r) => Either::B(r.call(req)),
        })
    }
}

#[derive(Debug)]
pub struct EitherMiddlewareFuture<L, R>(Either<L, R>);

impl<L, R, LB, RB> Future for EitherMiddlewareFuture<L, R>
where
    L: Future<Item = http::Response<LB>>,
    R: Future<Item = http::Response<RB>, Error = L::Error>,
{
    type Item = http::Response<Either<LB, RB>>;
    type Error = L::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.0 {
            Either::A(ref mut l) => {
                let l = try_ready!(l.poll());
                let l = l.map(Either::A);
                Ok(Async::Ready(l))
            },
            Either::B(ref mut r) => {
                let r = try_ready!(r.poll());
                let r = r.map(Either::B);
                Ok(Async::Ready(r))
            }
        }
    }
}
