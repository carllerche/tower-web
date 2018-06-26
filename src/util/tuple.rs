//! Implementations of `Resource` for tuple types.

use response::{Context, IntoResponse, Serializer};
use routing::{self, RouteSet, RouteMatch};
use service::Resource;
use util::{BufStream, Chain};

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

    fn dispatch<In: BufStream>(&mut self, _: (), _: &RouteMatch, _: &http::Request<()>, _: In) -> Self::Future {
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

    fn dispatch<In: BufStream>(&mut self,
                               destination: Self::Destination,
                               route_match: &RouteMatch,
                               request: &http::Request<()>,
                               payload: In)
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
// ===== 3 =====

#[derive(Clone)]
pub enum Either3<A, B, C> {
    A(A),
    B(B),
    C(C),
}

impl<A, B, C> Future for Either3<A, B, C>
where
    A: Future,
    B: Future<Error = A::Error>,
    C: Future<Error = A::Error>,
{
    type Item = Either3<A::Item, B::Item, C::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either3::*;

        match *self {
            A(ref mut f) => Ok(Either3::A(try_ready!(f.poll())).into()),
            B(ref mut f) => Ok(Either3::B(try_ready!(f.poll())).into()),
            C(ref mut f) => Ok(Either3::C(try_ready!(f.poll())).into()),
        }
    }
}

impl<A, B, C> Stream for Either3<A, B, C>
where
    A: Stream,
    B: Stream<Error = A::Error>,
    C: Stream<Error = A::Error>,
{
    type Item = Either3<A::Item, B::Item, C::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either3::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(Either3::A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(Either3::B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(Either3::C).into()),
        }
    }
}

impl<A, B, C> Buf for Either3<A, B, C>
where
    A: Buf,
    B: Buf,
    C: Buf,
{
    fn remaining(&self) -> usize {
        use self::Either3::*;

        match *self {
            A(ref b) => b.remaining(),
            B(ref b) => b.remaining(),
            C(ref b) => b.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        use self::Either3::*;

        match *self {
            A(ref b) => b.bytes(),
            B(ref b) => b.bytes(),
            C(ref b) => b.bytes(),
        }
    }

    fn advance(&mut self, cnt: usize) {
        use self::Either3::*;

        match *self {
            A(ref mut b) => b.advance(cnt),
            B(ref mut b) => b.advance(cnt),
            C(ref mut b) => b.advance(cnt),
        }
    }
}

impl<A, B, C> IntoResponse for Either3<A, B, C>
where
    A: IntoResponse,
    B: IntoResponse,
    C: IntoResponse,
{
    type Buf = Either3<A::Buf, B::Buf, C::Buf>;
    type Body = Either3<A::Body, B::Body, C::Body>;

    fn into_response<S>(self, context: &Context<S>) ->  http::Response<Self::Body>
    where S: Serializer
    {
        use self::Either3::*;

        match self {
            A(r) => r.into_response(context).map(Either3::A),
            B(r) => r.into_response(context).map(Either3::B),
            C(r) => r.into_response(context).map(Either3::C),
        }
    }
}

impl<R0, R1, R2> Resource for (R0, R1, R2)
where
    R0: Resource,
    R1: Resource,
    R2: Resource,
{
    type Destination = Either3<R0::Destination, R1::Destination, R2::Destination>;
    type Buf = Either3<R0::Buf, R1::Buf, R2::Buf>;
    type Body = Either3<R0::Body, R1::Body, R2::Body>;
    type Response = Either3<R0::Response, R1::Response, R2::Response>;
    type Future = Either3<R0::Future, R1::Future, R2::Future>;

    fn routes<S>(&self, serializer: &S) -> RouteSet<Self::Destination, S::ContentType>
    where S: Serializer,
    {
        let mut routes = routing::Builder::new();

        for route in self.0.routes(serializer) {
            routes.push(route.map(Either3::A));
        }

        for route in self.1.routes(serializer) {
            routes.push(route.map(Either3::B));
        }

        for route in self.2.routes(serializer) {
            routes.push(route.map(Either3::C));
        }

        routes.build()
    }

    fn dispatch<In: BufStream>(&mut self,
                               destination: Self::Destination,
                               route_match: &RouteMatch,
                               request: &http::Request<()>,
                               payload: In)
        -> Self::Future
    {
        use self::Either3::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, request, payload))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, request, payload))
            }
            C(d) => {
                C(self.2.dispatch(d, route_match, request, payload))
            }
        }
    }
}

impl<R0, R1, R2, U> Chain<U> for (R0, R1, R2) {
    type Output = (R0, R1, R2, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, self.1, self.2, other)
    }
}
