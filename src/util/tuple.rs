//! Implementations of `Resource` for tuple types.

use response::{Context, IntoResponse, Serializer};
use routing::{self, RouteSet, RouteMatch};
use service::{Payload, Resource};
use util::Chain;

use bytes::{Bytes, Buf};
use futures::{Future, Stream, Poll};
use futures::future::FutureResult;
use futures::stream::Once;
use http;

use std::io;

// ===== 0 =====

impl Resource for () {
    type Destination = ();
    type Buf = io::Cursor<Bytes>;
    type Body = Once<Self::Buf, ::Error>;
    type Response = String;
    type Future = FutureResult<String, ::Error>;

    fn routes<S>(&self, _: &S) -> RouteSet<(), S::ContentType>
    where S: Serializer,
    {
        RouteSet::new()
    }

    fn dispatch<T: Payload>(&mut self, _: (), _: &RouteMatch, _: &http::Request<()>, _: T) -> Self::Future {
        unreachable!();
    }
}

impl<U> Chain<U> for () {
    type Output = U;

    fn chain(self, other: U) -> Self::Output {
        other
    }
}
// ===== 2 =====

#[derive(Clone)]
pub enum Either2<A, B> {
    A(A),
    B(B),
}

impl<A, B> Future for Either2<A, B>
where
    A: Future,
    B: Future<Error = A::Error>,
{
    type Item = Either2<A::Item, B::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either2::*;

        match *self {
            A(ref mut f) => Ok(Either2::A(try_ready!(f.poll())).into()),
            B(ref mut f) => Ok(Either2::B(try_ready!(f.poll())).into()),
        }
    }
}

impl<A, B> Stream for Either2<A, B>
where
    A: Stream,
    B: Stream<Error = A::Error>,
{
    type Item = Either2<A::Item, B::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either2::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(Either2::A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(Either2::B).into()),
        }
    }
}

impl<A, B> Buf for Either2<A, B>
where
    A: Buf,
    B: Buf,
{
    fn remaining(&self) -> usize {
        use self::Either2::*;

        match *self {
            A(ref b) => b.remaining(),
            B(ref b) => b.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        use self::Either2::*;

        match *self {
            A(ref b) => b.bytes(),
            B(ref b) => b.bytes(),
        }
    }

    fn advance(&mut self, cnt: usize) {
        use self::Either2::*;

        match *self {
            A(ref mut b) => b.advance(cnt),
            B(ref mut b) => b.advance(cnt),
        }
    }
}

impl<A, B> IntoResponse for Either2<A, B>
where
    A: IntoResponse,
    B: IntoResponse,
{
    type Buf = Either2<A::Buf, B::Buf>;
    type Body = Either2<A::Body, B::Body>;

    fn into_response<S>(self, context: &Context<S>) ->  http::Response<Self::Body>
    where S: Serializer
    {
        use self::Either2::*;

        match self {
            A(r) => r.into_response(context).map(Either2::A),
            B(r) => r.into_response(context).map(Either2::B),
        }
    }
}

impl<R0, R1> Resource for (R0, R1)
where
    R0: Resource,
    R1: Resource,
{
    type Destination = Either2<R0::Destination, R1::Destination>;
    type Buf = Either2<R0::Buf, R1::Buf>;
    type Body = Either2<R0::Body, R1::Body>;
    type Response = Either2<R0::Response, R1::Response>;
    type Future = Either2<R0::Future, R1::Future>;

    fn routes<S>(&self, serializer: &S) -> RouteSet<Self::Destination, S::ContentType>
    where S: Serializer,
    {
        let mut routes = routing::Builder::new();

        for route in self.0.routes(serializer) {
            routes.push(route.map(Either2::A));
        }

        for route in self.1.routes(serializer) {
            routes.push(route.map(Either2::B));
        }

        routes.build()
    }

    fn dispatch<T: Payload>(&mut self,
                            destination: Self::Destination,
                            route_match: &RouteMatch,
                            request: &http::Request<()>,
                            payload: T,)
        -> Self::Future
    {
        use self::Either2::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, request, payload))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, request, payload))
            }
        }
    }
}

impl<R0, R1, U> Chain<U> for (R0, R1) {
    type Output = (R0, R1, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, self.1, other)
    }
}
