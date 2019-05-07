//! Implementations of `Resource` for tuple types.

// NOTE: This file should not be updated directly. Instead, update
// `util/gen-tuple.rs` and regenerate this file.

use crate::extract::{self, ExtractFuture};
use crate::response::{Context, Response, Serializer};
use crate::routing::{self, Resource, ResourceFuture, IntoResource, RouteSet, RouteMatch};
use crate::util::{BufStream, Chain};
use crate::util::http::{HttpFuture, SealedFuture};

use bytes::Buf;
use futures::{Future, Stream, Async, Poll};
use http;

// ===== Utility traits =====



// ===== 0 =====

#[derive(Debug)]
pub struct Join0 {
    _p: (),
}

impl Join0 {
    pub fn new() -> Self {
        Self { _p: () }
    }

    pub fn into_inner(self) -> () {
        ()
    }
}

impl Future for Join0 {
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
        Ok(().into())
    }
}

impl<U> Chain<U> for () {
    type Output = U;

    fn chain(self, other: U) -> Self::Output {
        other
    }
}
// ===== 1 =====

#[derive(Debug, Clone)]
pub enum Either1<A> {
    A(A),
}

impl<A> Future for Either1<A>
where
    A: Future,
{
    type Item = Either1<A::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either1::*;

        match *self {
            A(ref mut f) => Ok(A(try_ready!(f.poll())).into()),
        }
    }
}

impl<A> ResourceFuture for Either1<A>
where
    A: ResourceFuture,
{
    type Body = Either1<A::Body>;

    fn poll_response(&mut self, request: &http::Request<()>) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either1::*;

        let response = match *self {
            A(ref mut f) => try_ready!(f.poll_response(request)).map(A),
        };
        Ok(response.into())
    }
}

impl<A> Either1<A>
where
    A: ExtractFuture,
{

    pub fn poll_ready(&mut self) -> Poll<(), extract::Error> {
        use self::Either1::*;

        match *self {
            A(ref mut f) => f.poll(),
        }
    }
}

impl<A> HttpFuture for Either1<A>
where
    A: HttpFuture,
{
    type Body = Either1<A::Body>;

    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either1::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http()).map(A).into()),
        }
    }
}

impl<A> SealedFuture for Either1<A>
where
    A: HttpFuture,
{
}

impl<A> Stream for Either1<A>
where
    A: Stream,
{
    type Item = Either1<A::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either1::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
        }
    }
}

impl<A> BufStream for Either1<A>
where
    A: BufStream,
{
    type Item = Either1<A::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either1::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
        }
    }
}

impl<A> Buf for Either1<A>
where
    A: Buf,
{
    fn remaining(&self) -> usize {
        use self::Either1::*;

        match *self {
            A(ref b) => b.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        use self::Either1::*;

        match *self {
            A(ref b) => b.bytes(),
        }
    }

    fn advance(&mut self, cnt: usize) {
        use self::Either1::*;

        match *self {
            A(ref mut b) => b.advance(cnt),
        }
    }
}

impl<A> Response for Either1<A>
where
    A: Response,
{
    type Buf = Either1<A::Buf>;
    type Body = Either1<A::Body>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where S: Serializer
    {
        use self::Either1::*;

        match self {
            A(r) => Ok(r.into_http(context)?.map(Either1::A)),
        }
    }
}

impl<R0> Resource for (R0,)
where
    R0: Resource,
{
    type Destination = Either1<R0::Destination>;
    type RequestBody = R0::RequestBody;
    type Buf = Either1<R0::Buf>;
    type Body = Either1<R0::Body>;
    type Future = Either1<R0::Future>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: &RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either1::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
        }
    }
}
// ===== 2 =====

#[derive(Debug, Clone)]
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
            A(ref mut f) => Ok(A(try_ready!(f.poll())).into()),
            B(ref mut f) => Ok(B(try_ready!(f.poll())).into()),
        }
    }
}

impl<A, B> ResourceFuture for Either2<A, B>
where
    A: ResourceFuture,
    B: ResourceFuture,
{
    type Body = Either2<A::Body, B::Body>;

    fn poll_response(&mut self, request: &http::Request<()>) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either2::*;

        let response = match *self {
            A(ref mut f) => try_ready!(f.poll_response(request)).map(A),
            B(ref mut f) => try_ready!(f.poll_response(request)).map(B),
        };
        Ok(response.into())
    }
}

impl<A, B> Either2<A, B>
where
    A: ExtractFuture,
    B: ExtractFuture,
{

    pub fn poll_ready(&mut self) -> Poll<(), extract::Error> {
        use self::Either2::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
        }
    }
}

impl<A, B> HttpFuture for Either2<A, B>
where
    A: HttpFuture,
    B: HttpFuture,
{
    type Body = Either2<A::Body, B::Body>;

    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either2::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll_http()).map(B).into()),
        }
    }
}

impl<A, B> SealedFuture for Either2<A, B>
where
    A: HttpFuture,
    B: HttpFuture,
{
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
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
        }
    }
}

impl<A, B> BufStream for Either2<A, B>
where
    A: BufStream,
    B: BufStream<Error = A::Error>,
{
    type Item = Either2<A::Item, B::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either2::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
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

impl<A, B> Response for Either2<A, B>
where
    A: Response,
    B: Response,
{
    type Buf = Either2<A::Buf, B::Buf>;
    type Body = Either2<A::Body, B::Body>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where S: Serializer
    {
        use self::Either2::*;

        match self {
            A(r) => Ok(r.into_http(context)?.map(Either2::A)),
            B(r) => Ok(r.into_http(context)?.map(Either2::B)),
        }
    }
}

impl<R0, R1> Resource for (R0, R1,)
where
    R0: Resource,
    R1: Resource<RequestBody = R0::RequestBody>,
{
    type Destination = Either2<R0::Destination, R1::Destination>;
    type RequestBody = R0::RequestBody;
    type Buf = Either2<R0::Buf, R1::Buf>;
    type Body = Either2<R0::Body, R1::Body>;
    type Future = Either2<R0::Future, R1::Future>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: &RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either2::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, body))
            }
        }
    }
}
// ===== 3 =====

#[derive(Debug, Clone)]
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
            A(ref mut f) => Ok(A(try_ready!(f.poll())).into()),
            B(ref mut f) => Ok(B(try_ready!(f.poll())).into()),
            C(ref mut f) => Ok(C(try_ready!(f.poll())).into()),
        }
    }
}

impl<A, B, C> ResourceFuture for Either3<A, B, C>
where
    A: ResourceFuture,
    B: ResourceFuture,
    C: ResourceFuture,
{
    type Body = Either3<A::Body, B::Body, C::Body>;

    fn poll_response(&mut self, request: &http::Request<()>) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either3::*;

        let response = match *self {
            A(ref mut f) => try_ready!(f.poll_response(request)).map(A),
            B(ref mut f) => try_ready!(f.poll_response(request)).map(B),
            C(ref mut f) => try_ready!(f.poll_response(request)).map(C),
        };
        Ok(response.into())
    }
}

impl<A, B, C> Either3<A, B, C>
where
    A: ExtractFuture,
    B: ExtractFuture,
    C: ExtractFuture,
{

    pub fn poll_ready(&mut self) -> Poll<(), extract::Error> {
        use self::Either3::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
            C(ref mut f) => f.poll(),
        }
    }
}

impl<A, B, C> HttpFuture for Either3<A, B, C>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
{
    type Body = Either3<A::Body, B::Body, C::Body>;

    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either3::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll_http()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll_http()).map(C).into()),
        }
    }
}

impl<A, B, C> SealedFuture for Either3<A, B, C>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
{
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
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
        }
    }
}

impl<A, B, C> BufStream for Either3<A, B, C>
where
    A: BufStream,
    B: BufStream<Error = A::Error>,
    C: BufStream<Error = A::Error>,
{
    type Item = Either3<A::Item, B::Item, C::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either3::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
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

impl<A, B, C> Response for Either3<A, B, C>
where
    A: Response,
    B: Response,
    C: Response,
{
    type Buf = Either3<A::Buf, B::Buf, C::Buf>;
    type Body = Either3<A::Body, B::Body, C::Body>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where S: Serializer
    {
        use self::Either3::*;

        match self {
            A(r) => Ok(r.into_http(context)?.map(Either3::A)),
            B(r) => Ok(r.into_http(context)?.map(Either3::B)),
            C(r) => Ok(r.into_http(context)?.map(Either3::C)),
        }
    }
}

impl<R0, R1, R2> Resource for (R0, R1, R2,)
where
    R0: Resource,
    R1: Resource<RequestBody = R0::RequestBody>,
    R2: Resource<RequestBody = R0::RequestBody>,
{
    type Destination = Either3<R0::Destination, R1::Destination, R2::Destination>;
    type RequestBody = R0::RequestBody;
    type Buf = Either3<R0::Buf, R1::Buf, R2::Buf>;
    type Body = Either3<R0::Body, R1::Body, R2::Body>;
    type Future = Either3<R0::Future, R1::Future, R2::Future>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: &RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either3::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, body))
            }
            C(d) => {
                C(self.2.dispatch(d, route_match, body))
            }
        }
    }
}
// ===== 4 =====

#[derive(Debug, Clone)]
pub enum Either4<A, B, C, D> {
    A(A),
    B(B),
    C(C),
    D(D),
}

impl<A, B, C, D> Future for Either4<A, B, C, D>
where
    A: Future,
    B: Future<Error = A::Error>,
    C: Future<Error = A::Error>,
    D: Future<Error = A::Error>,
{
    type Item = Either4<A::Item, B::Item, C::Item, D::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either4::*;

        match *self {
            A(ref mut f) => Ok(A(try_ready!(f.poll())).into()),
            B(ref mut f) => Ok(B(try_ready!(f.poll())).into()),
            C(ref mut f) => Ok(C(try_ready!(f.poll())).into()),
            D(ref mut f) => Ok(D(try_ready!(f.poll())).into()),
        }
    }
}

impl<A, B, C, D> ResourceFuture for Either4<A, B, C, D>
where
    A: ResourceFuture,
    B: ResourceFuture,
    C: ResourceFuture,
    D: ResourceFuture,
{
    type Body = Either4<A::Body, B::Body, C::Body, D::Body>;

    fn poll_response(&mut self, request: &http::Request<()>) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either4::*;

        let response = match *self {
            A(ref mut f) => try_ready!(f.poll_response(request)).map(A),
            B(ref mut f) => try_ready!(f.poll_response(request)).map(B),
            C(ref mut f) => try_ready!(f.poll_response(request)).map(C),
            D(ref mut f) => try_ready!(f.poll_response(request)).map(D),
        };
        Ok(response.into())
    }
}

impl<A, B, C, D> Either4<A, B, C, D>
where
    A: ExtractFuture,
    B: ExtractFuture,
    C: ExtractFuture,
    D: ExtractFuture,
{

    pub fn poll_ready(&mut self) -> Poll<(), extract::Error> {
        use self::Either4::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
            C(ref mut f) => f.poll(),
            D(ref mut f) => f.poll(),
        }
    }
}

impl<A, B, C, D> HttpFuture for Either4<A, B, C, D>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
{
    type Body = Either4<A::Body, B::Body, C::Body, D::Body>;

    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either4::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll_http()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll_http()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll_http()).map(D).into()),
        }
    }
}

impl<A, B, C, D> SealedFuture for Either4<A, B, C, D>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
{
}

impl<A, B, C, D> Stream for Either4<A, B, C, D>
where
    A: Stream,
    B: Stream<Error = A::Error>,
    C: Stream<Error = A::Error>,
    D: Stream<Error = A::Error>,
{
    type Item = Either4<A::Item, B::Item, C::Item, D::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either4::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
        }
    }
}

impl<A, B, C, D> BufStream for Either4<A, B, C, D>
where
    A: BufStream,
    B: BufStream<Error = A::Error>,
    C: BufStream<Error = A::Error>,
    D: BufStream<Error = A::Error>,
{
    type Item = Either4<A::Item, B::Item, C::Item, D::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either4::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
        }
    }
}

impl<A, B, C, D> Buf for Either4<A, B, C, D>
where
    A: Buf,
    B: Buf,
    C: Buf,
    D: Buf,
{
    fn remaining(&self) -> usize {
        use self::Either4::*;

        match *self {
            A(ref b) => b.remaining(),
            B(ref b) => b.remaining(),
            C(ref b) => b.remaining(),
            D(ref b) => b.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        use self::Either4::*;

        match *self {
            A(ref b) => b.bytes(),
            B(ref b) => b.bytes(),
            C(ref b) => b.bytes(),
            D(ref b) => b.bytes(),
        }
    }

    fn advance(&mut self, cnt: usize) {
        use self::Either4::*;

        match *self {
            A(ref mut b) => b.advance(cnt),
            B(ref mut b) => b.advance(cnt),
            C(ref mut b) => b.advance(cnt),
            D(ref mut b) => b.advance(cnt),
        }
    }
}

impl<A, B, C, D> Response for Either4<A, B, C, D>
where
    A: Response,
    B: Response,
    C: Response,
    D: Response,
{
    type Buf = Either4<A::Buf, B::Buf, C::Buf, D::Buf>;
    type Body = Either4<A::Body, B::Body, C::Body, D::Body>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where S: Serializer
    {
        use self::Either4::*;

        match self {
            A(r) => Ok(r.into_http(context)?.map(Either4::A)),
            B(r) => Ok(r.into_http(context)?.map(Either4::B)),
            C(r) => Ok(r.into_http(context)?.map(Either4::C)),
            D(r) => Ok(r.into_http(context)?.map(Either4::D)),
        }
    }
}

impl<R0, R1, R2, R3> Resource for (R0, R1, R2, R3,)
where
    R0: Resource,
    R1: Resource<RequestBody = R0::RequestBody>,
    R2: Resource<RequestBody = R0::RequestBody>,
    R3: Resource<RequestBody = R0::RequestBody>,
{
    type Destination = Either4<R0::Destination, R1::Destination, R2::Destination, R3::Destination>;
    type RequestBody = R0::RequestBody;
    type Buf = Either4<R0::Buf, R1::Buf, R2::Buf, R3::Buf>;
    type Body = Either4<R0::Body, R1::Body, R2::Body, R3::Body>;
    type Future = Either4<R0::Future, R1::Future, R2::Future, R3::Future>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: &RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either4::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, body))
            }
            C(d) => {
                C(self.2.dispatch(d, route_match, body))
            }
            D(d) => {
                D(self.3.dispatch(d, route_match, body))
            }
        }
    }
}
// ===== 5 =====

#[derive(Debug, Clone)]
pub enum Either5<A, B, C, D, E> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
}

impl<A, B, C, D, E> Future for Either5<A, B, C, D, E>
where
    A: Future,
    B: Future<Error = A::Error>,
    C: Future<Error = A::Error>,
    D: Future<Error = A::Error>,
    E: Future<Error = A::Error>,
{
    type Item = Either5<A::Item, B::Item, C::Item, D::Item, E::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either5::*;

        match *self {
            A(ref mut f) => Ok(A(try_ready!(f.poll())).into()),
            B(ref mut f) => Ok(B(try_ready!(f.poll())).into()),
            C(ref mut f) => Ok(C(try_ready!(f.poll())).into()),
            D(ref mut f) => Ok(D(try_ready!(f.poll())).into()),
            E(ref mut f) => Ok(E(try_ready!(f.poll())).into()),
        }
    }
}

impl<A, B, C, D, E> ResourceFuture for Either5<A, B, C, D, E>
where
    A: ResourceFuture,
    B: ResourceFuture,
    C: ResourceFuture,
    D: ResourceFuture,
    E: ResourceFuture,
{
    type Body = Either5<A::Body, B::Body, C::Body, D::Body, E::Body>;

    fn poll_response(&mut self, request: &http::Request<()>) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either5::*;

        let response = match *self {
            A(ref mut f) => try_ready!(f.poll_response(request)).map(A),
            B(ref mut f) => try_ready!(f.poll_response(request)).map(B),
            C(ref mut f) => try_ready!(f.poll_response(request)).map(C),
            D(ref mut f) => try_ready!(f.poll_response(request)).map(D),
            E(ref mut f) => try_ready!(f.poll_response(request)).map(E),
        };
        Ok(response.into())
    }
}

impl<A, B, C, D, E> Either5<A, B, C, D, E>
where
    A: ExtractFuture,
    B: ExtractFuture,
    C: ExtractFuture,
    D: ExtractFuture,
    E: ExtractFuture,
{

    pub fn poll_ready(&mut self) -> Poll<(), extract::Error> {
        use self::Either5::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
            C(ref mut f) => f.poll(),
            D(ref mut f) => f.poll(),
            E(ref mut f) => f.poll(),
        }
    }
}

impl<A, B, C, D, E> HttpFuture for Either5<A, B, C, D, E>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
{
    type Body = Either5<A::Body, B::Body, C::Body, D::Body, E::Body>;

    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either5::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll_http()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll_http()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll_http()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll_http()).map(E).into()),
        }
    }
}

impl<A, B, C, D, E> SealedFuture for Either5<A, B, C, D, E>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
{
}

impl<A, B, C, D, E> Stream for Either5<A, B, C, D, E>
where
    A: Stream,
    B: Stream<Error = A::Error>,
    C: Stream<Error = A::Error>,
    D: Stream<Error = A::Error>,
    E: Stream<Error = A::Error>,
{
    type Item = Either5<A::Item, B::Item, C::Item, D::Item, E::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either5::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
        }
    }
}

impl<A, B, C, D, E> BufStream for Either5<A, B, C, D, E>
where
    A: BufStream,
    B: BufStream<Error = A::Error>,
    C: BufStream<Error = A::Error>,
    D: BufStream<Error = A::Error>,
    E: BufStream<Error = A::Error>,
{
    type Item = Either5<A::Item, B::Item, C::Item, D::Item, E::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either5::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
        }
    }
}

impl<A, B, C, D, E> Buf for Either5<A, B, C, D, E>
where
    A: Buf,
    B: Buf,
    C: Buf,
    D: Buf,
    E: Buf,
{
    fn remaining(&self) -> usize {
        use self::Either5::*;

        match *self {
            A(ref b) => b.remaining(),
            B(ref b) => b.remaining(),
            C(ref b) => b.remaining(),
            D(ref b) => b.remaining(),
            E(ref b) => b.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        use self::Either5::*;

        match *self {
            A(ref b) => b.bytes(),
            B(ref b) => b.bytes(),
            C(ref b) => b.bytes(),
            D(ref b) => b.bytes(),
            E(ref b) => b.bytes(),
        }
    }

    fn advance(&mut self, cnt: usize) {
        use self::Either5::*;

        match *self {
            A(ref mut b) => b.advance(cnt),
            B(ref mut b) => b.advance(cnt),
            C(ref mut b) => b.advance(cnt),
            D(ref mut b) => b.advance(cnt),
            E(ref mut b) => b.advance(cnt),
        }
    }
}

impl<A, B, C, D, E> Response for Either5<A, B, C, D, E>
where
    A: Response,
    B: Response,
    C: Response,
    D: Response,
    E: Response,
{
    type Buf = Either5<A::Buf, B::Buf, C::Buf, D::Buf, E::Buf>;
    type Body = Either5<A::Body, B::Body, C::Body, D::Body, E::Body>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where S: Serializer
    {
        use self::Either5::*;

        match self {
            A(r) => Ok(r.into_http(context)?.map(Either5::A)),
            B(r) => Ok(r.into_http(context)?.map(Either5::B)),
            C(r) => Ok(r.into_http(context)?.map(Either5::C)),
            D(r) => Ok(r.into_http(context)?.map(Either5::D)),
            E(r) => Ok(r.into_http(context)?.map(Either5::E)),
        }
    }
}

impl<R0, R1, R2, R3, R4> Resource for (R0, R1, R2, R3, R4,)
where
    R0: Resource,
    R1: Resource<RequestBody = R0::RequestBody>,
    R2: Resource<RequestBody = R0::RequestBody>,
    R3: Resource<RequestBody = R0::RequestBody>,
    R4: Resource<RequestBody = R0::RequestBody>,
{
    type Destination = Either5<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination>;
    type RequestBody = R0::RequestBody;
    type Buf = Either5<R0::Buf, R1::Buf, R2::Buf, R3::Buf, R4::Buf>;
    type Body = Either5<R0::Body, R1::Body, R2::Body, R3::Body, R4::Body>;
    type Future = Either5<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: &RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either5::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, body))
            }
            C(d) => {
                C(self.2.dispatch(d, route_match, body))
            }
            D(d) => {
                D(self.3.dispatch(d, route_match, body))
            }
            E(d) => {
                E(self.4.dispatch(d, route_match, body))
            }
        }
    }
}
// ===== 6 =====

#[derive(Debug, Clone)]
pub enum Either6<A, B, C, D, E, F> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
}

impl<A, B, C, D, E, F> Future for Either6<A, B, C, D, E, F>
where
    A: Future,
    B: Future<Error = A::Error>,
    C: Future<Error = A::Error>,
    D: Future<Error = A::Error>,
    E: Future<Error = A::Error>,
    F: Future<Error = A::Error>,
{
    type Item = Either6<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either6::*;

        match *self {
            A(ref mut f) => Ok(A(try_ready!(f.poll())).into()),
            B(ref mut f) => Ok(B(try_ready!(f.poll())).into()),
            C(ref mut f) => Ok(C(try_ready!(f.poll())).into()),
            D(ref mut f) => Ok(D(try_ready!(f.poll())).into()),
            E(ref mut f) => Ok(E(try_ready!(f.poll())).into()),
            F(ref mut f) => Ok(F(try_ready!(f.poll())).into()),
        }
    }
}

impl<A, B, C, D, E, F> ResourceFuture for Either6<A, B, C, D, E, F>
where
    A: ResourceFuture,
    B: ResourceFuture,
    C: ResourceFuture,
    D: ResourceFuture,
    E: ResourceFuture,
    F: ResourceFuture,
{
    type Body = Either6<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body>;

    fn poll_response(&mut self, request: &http::Request<()>) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either6::*;

        let response = match *self {
            A(ref mut f) => try_ready!(f.poll_response(request)).map(A),
            B(ref mut f) => try_ready!(f.poll_response(request)).map(B),
            C(ref mut f) => try_ready!(f.poll_response(request)).map(C),
            D(ref mut f) => try_ready!(f.poll_response(request)).map(D),
            E(ref mut f) => try_ready!(f.poll_response(request)).map(E),
            F(ref mut f) => try_ready!(f.poll_response(request)).map(F),
        };
        Ok(response.into())
    }
}

impl<A, B, C, D, E, F> Either6<A, B, C, D, E, F>
where
    A: ExtractFuture,
    B: ExtractFuture,
    C: ExtractFuture,
    D: ExtractFuture,
    E: ExtractFuture,
    F: ExtractFuture,
{

    pub fn poll_ready(&mut self) -> Poll<(), extract::Error> {
        use self::Either6::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
            C(ref mut f) => f.poll(),
            D(ref mut f) => f.poll(),
            E(ref mut f) => f.poll(),
            F(ref mut f) => f.poll(),
        }
    }
}

impl<A, B, C, D, E, F> HttpFuture for Either6<A, B, C, D, E, F>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
{
    type Body = Either6<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body>;

    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either6::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll_http()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll_http()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll_http()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll_http()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll_http()).map(F).into()),
        }
    }
}

impl<A, B, C, D, E, F> SealedFuture for Either6<A, B, C, D, E, F>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
{
}

impl<A, B, C, D, E, F> Stream for Either6<A, B, C, D, E, F>
where
    A: Stream,
    B: Stream<Error = A::Error>,
    C: Stream<Error = A::Error>,
    D: Stream<Error = A::Error>,
    E: Stream<Error = A::Error>,
    F: Stream<Error = A::Error>,
{
    type Item = Either6<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either6::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
        }
    }
}

impl<A, B, C, D, E, F> BufStream for Either6<A, B, C, D, E, F>
where
    A: BufStream,
    B: BufStream<Error = A::Error>,
    C: BufStream<Error = A::Error>,
    D: BufStream<Error = A::Error>,
    E: BufStream<Error = A::Error>,
    F: BufStream<Error = A::Error>,
{
    type Item = Either6<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either6::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
        }
    }
}

impl<A, B, C, D, E, F> Buf for Either6<A, B, C, D, E, F>
where
    A: Buf,
    B: Buf,
    C: Buf,
    D: Buf,
    E: Buf,
    F: Buf,
{
    fn remaining(&self) -> usize {
        use self::Either6::*;

        match *self {
            A(ref b) => b.remaining(),
            B(ref b) => b.remaining(),
            C(ref b) => b.remaining(),
            D(ref b) => b.remaining(),
            E(ref b) => b.remaining(),
            F(ref b) => b.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        use self::Either6::*;

        match *self {
            A(ref b) => b.bytes(),
            B(ref b) => b.bytes(),
            C(ref b) => b.bytes(),
            D(ref b) => b.bytes(),
            E(ref b) => b.bytes(),
            F(ref b) => b.bytes(),
        }
    }

    fn advance(&mut self, cnt: usize) {
        use self::Either6::*;

        match *self {
            A(ref mut b) => b.advance(cnt),
            B(ref mut b) => b.advance(cnt),
            C(ref mut b) => b.advance(cnt),
            D(ref mut b) => b.advance(cnt),
            E(ref mut b) => b.advance(cnt),
            F(ref mut b) => b.advance(cnt),
        }
    }
}

impl<A, B, C, D, E, F> Response for Either6<A, B, C, D, E, F>
where
    A: Response,
    B: Response,
    C: Response,
    D: Response,
    E: Response,
    F: Response,
{
    type Buf = Either6<A::Buf, B::Buf, C::Buf, D::Buf, E::Buf, F::Buf>;
    type Body = Either6<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where S: Serializer
    {
        use self::Either6::*;

        match self {
            A(r) => Ok(r.into_http(context)?.map(Either6::A)),
            B(r) => Ok(r.into_http(context)?.map(Either6::B)),
            C(r) => Ok(r.into_http(context)?.map(Either6::C)),
            D(r) => Ok(r.into_http(context)?.map(Either6::D)),
            E(r) => Ok(r.into_http(context)?.map(Either6::E)),
            F(r) => Ok(r.into_http(context)?.map(Either6::F)),
        }
    }
}

impl<R0, R1, R2, R3, R4, R5> Resource for (R0, R1, R2, R3, R4, R5,)
where
    R0: Resource,
    R1: Resource<RequestBody = R0::RequestBody>,
    R2: Resource<RequestBody = R0::RequestBody>,
    R3: Resource<RequestBody = R0::RequestBody>,
    R4: Resource<RequestBody = R0::RequestBody>,
    R5: Resource<RequestBody = R0::RequestBody>,
{
    type Destination = Either6<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination, R5::Destination>;
    type RequestBody = R0::RequestBody;
    type Buf = Either6<R0::Buf, R1::Buf, R2::Buf, R3::Buf, R4::Buf, R5::Buf>;
    type Body = Either6<R0::Body, R1::Body, R2::Body, R3::Body, R4::Body, R5::Body>;
    type Future = Either6<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future, R5::Future>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: &RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either6::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, body))
            }
            C(d) => {
                C(self.2.dispatch(d, route_match, body))
            }
            D(d) => {
                D(self.3.dispatch(d, route_match, body))
            }
            E(d) => {
                E(self.4.dispatch(d, route_match, body))
            }
            F(d) => {
                F(self.5.dispatch(d, route_match, body))
            }
        }
    }
}
// ===== 7 =====

#[derive(Debug, Clone)]
pub enum Either7<A, B, C, D, E, F, G> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
}

impl<A, B, C, D, E, F, G> Future for Either7<A, B, C, D, E, F, G>
where
    A: Future,
    B: Future<Error = A::Error>,
    C: Future<Error = A::Error>,
    D: Future<Error = A::Error>,
    E: Future<Error = A::Error>,
    F: Future<Error = A::Error>,
    G: Future<Error = A::Error>,
{
    type Item = Either7<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either7::*;

        match *self {
            A(ref mut f) => Ok(A(try_ready!(f.poll())).into()),
            B(ref mut f) => Ok(B(try_ready!(f.poll())).into()),
            C(ref mut f) => Ok(C(try_ready!(f.poll())).into()),
            D(ref mut f) => Ok(D(try_ready!(f.poll())).into()),
            E(ref mut f) => Ok(E(try_ready!(f.poll())).into()),
            F(ref mut f) => Ok(F(try_ready!(f.poll())).into()),
            G(ref mut f) => Ok(G(try_ready!(f.poll())).into()),
        }
    }
}

impl<A, B, C, D, E, F, G> ResourceFuture for Either7<A, B, C, D, E, F, G>
where
    A: ResourceFuture,
    B: ResourceFuture,
    C: ResourceFuture,
    D: ResourceFuture,
    E: ResourceFuture,
    F: ResourceFuture,
    G: ResourceFuture,
{
    type Body = Either7<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body>;

    fn poll_response(&mut self, request: &http::Request<()>) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either7::*;

        let response = match *self {
            A(ref mut f) => try_ready!(f.poll_response(request)).map(A),
            B(ref mut f) => try_ready!(f.poll_response(request)).map(B),
            C(ref mut f) => try_ready!(f.poll_response(request)).map(C),
            D(ref mut f) => try_ready!(f.poll_response(request)).map(D),
            E(ref mut f) => try_ready!(f.poll_response(request)).map(E),
            F(ref mut f) => try_ready!(f.poll_response(request)).map(F),
            G(ref mut f) => try_ready!(f.poll_response(request)).map(G),
        };
        Ok(response.into())
    }
}

impl<A, B, C, D, E, F, G> Either7<A, B, C, D, E, F, G>
where
    A: ExtractFuture,
    B: ExtractFuture,
    C: ExtractFuture,
    D: ExtractFuture,
    E: ExtractFuture,
    F: ExtractFuture,
    G: ExtractFuture,
{

    pub fn poll_ready(&mut self) -> Poll<(), extract::Error> {
        use self::Either7::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
            C(ref mut f) => f.poll(),
            D(ref mut f) => f.poll(),
            E(ref mut f) => f.poll(),
            F(ref mut f) => f.poll(),
            G(ref mut f) => f.poll(),
        }
    }
}

impl<A, B, C, D, E, F, G> HttpFuture for Either7<A, B, C, D, E, F, G>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
    G: HttpFuture,
{
    type Body = Either7<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body>;

    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either7::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll_http()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll_http()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll_http()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll_http()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll_http()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll_http()).map(G).into()),
        }
    }
}

impl<A, B, C, D, E, F, G> SealedFuture for Either7<A, B, C, D, E, F, G>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
    G: HttpFuture,
{
}

impl<A, B, C, D, E, F, G> Stream for Either7<A, B, C, D, E, F, G>
where
    A: Stream,
    B: Stream<Error = A::Error>,
    C: Stream<Error = A::Error>,
    D: Stream<Error = A::Error>,
    E: Stream<Error = A::Error>,
    F: Stream<Error = A::Error>,
    G: Stream<Error = A::Error>,
{
    type Item = Either7<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either7::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll()).map(G).into()),
        }
    }
}

impl<A, B, C, D, E, F, G> BufStream for Either7<A, B, C, D, E, F, G>
where
    A: BufStream,
    B: BufStream<Error = A::Error>,
    C: BufStream<Error = A::Error>,
    D: BufStream<Error = A::Error>,
    E: BufStream<Error = A::Error>,
    F: BufStream<Error = A::Error>,
    G: BufStream<Error = A::Error>,
{
    type Item = Either7<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either7::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll()).map(G).into()),
        }
    }
}

impl<A, B, C, D, E, F, G> Buf for Either7<A, B, C, D, E, F, G>
where
    A: Buf,
    B: Buf,
    C: Buf,
    D: Buf,
    E: Buf,
    F: Buf,
    G: Buf,
{
    fn remaining(&self) -> usize {
        use self::Either7::*;

        match *self {
            A(ref b) => b.remaining(),
            B(ref b) => b.remaining(),
            C(ref b) => b.remaining(),
            D(ref b) => b.remaining(),
            E(ref b) => b.remaining(),
            F(ref b) => b.remaining(),
            G(ref b) => b.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        use self::Either7::*;

        match *self {
            A(ref b) => b.bytes(),
            B(ref b) => b.bytes(),
            C(ref b) => b.bytes(),
            D(ref b) => b.bytes(),
            E(ref b) => b.bytes(),
            F(ref b) => b.bytes(),
            G(ref b) => b.bytes(),
        }
    }

    fn advance(&mut self, cnt: usize) {
        use self::Either7::*;

        match *self {
            A(ref mut b) => b.advance(cnt),
            B(ref mut b) => b.advance(cnt),
            C(ref mut b) => b.advance(cnt),
            D(ref mut b) => b.advance(cnt),
            E(ref mut b) => b.advance(cnt),
            F(ref mut b) => b.advance(cnt),
            G(ref mut b) => b.advance(cnt),
        }
    }
}

impl<A, B, C, D, E, F, G> Response for Either7<A, B, C, D, E, F, G>
where
    A: Response,
    B: Response,
    C: Response,
    D: Response,
    E: Response,
    F: Response,
    G: Response,
{
    type Buf = Either7<A::Buf, B::Buf, C::Buf, D::Buf, E::Buf, F::Buf, G::Buf>;
    type Body = Either7<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where S: Serializer
    {
        use self::Either7::*;

        match self {
            A(r) => Ok(r.into_http(context)?.map(Either7::A)),
            B(r) => Ok(r.into_http(context)?.map(Either7::B)),
            C(r) => Ok(r.into_http(context)?.map(Either7::C)),
            D(r) => Ok(r.into_http(context)?.map(Either7::D)),
            E(r) => Ok(r.into_http(context)?.map(Either7::E)),
            F(r) => Ok(r.into_http(context)?.map(Either7::F)),
            G(r) => Ok(r.into_http(context)?.map(Either7::G)),
        }
    }
}

impl<R0, R1, R2, R3, R4, R5, R6> Resource for (R0, R1, R2, R3, R4, R5, R6,)
where
    R0: Resource,
    R1: Resource<RequestBody = R0::RequestBody>,
    R2: Resource<RequestBody = R0::RequestBody>,
    R3: Resource<RequestBody = R0::RequestBody>,
    R4: Resource<RequestBody = R0::RequestBody>,
    R5: Resource<RequestBody = R0::RequestBody>,
    R6: Resource<RequestBody = R0::RequestBody>,
{
    type Destination = Either7<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination, R5::Destination, R6::Destination>;
    type RequestBody = R0::RequestBody;
    type Buf = Either7<R0::Buf, R1::Buf, R2::Buf, R3::Buf, R4::Buf, R5::Buf, R6::Buf>;
    type Body = Either7<R0::Body, R1::Body, R2::Body, R3::Body, R4::Body, R5::Body, R6::Body>;
    type Future = Either7<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future, R5::Future, R6::Future>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: &RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either7::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, body))
            }
            C(d) => {
                C(self.2.dispatch(d, route_match, body))
            }
            D(d) => {
                D(self.3.dispatch(d, route_match, body))
            }
            E(d) => {
                E(self.4.dispatch(d, route_match, body))
            }
            F(d) => {
                F(self.5.dispatch(d, route_match, body))
            }
            G(d) => {
                G(self.6.dispatch(d, route_match, body))
            }
        }
    }
}
// ===== 8 =====

#[derive(Debug, Clone)]
pub enum Either8<A, B, C, D, E, F, G, H> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
    H(H),
}

impl<A, B, C, D, E, F, G, H> Future for Either8<A, B, C, D, E, F, G, H>
where
    A: Future,
    B: Future<Error = A::Error>,
    C: Future<Error = A::Error>,
    D: Future<Error = A::Error>,
    E: Future<Error = A::Error>,
    F: Future<Error = A::Error>,
    G: Future<Error = A::Error>,
    H: Future<Error = A::Error>,
{
    type Item = Either8<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either8::*;

        match *self {
            A(ref mut f) => Ok(A(try_ready!(f.poll())).into()),
            B(ref mut f) => Ok(B(try_ready!(f.poll())).into()),
            C(ref mut f) => Ok(C(try_ready!(f.poll())).into()),
            D(ref mut f) => Ok(D(try_ready!(f.poll())).into()),
            E(ref mut f) => Ok(E(try_ready!(f.poll())).into()),
            F(ref mut f) => Ok(F(try_ready!(f.poll())).into()),
            G(ref mut f) => Ok(G(try_ready!(f.poll())).into()),
            H(ref mut f) => Ok(H(try_ready!(f.poll())).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H> ResourceFuture for Either8<A, B, C, D, E, F, G, H>
where
    A: ResourceFuture,
    B: ResourceFuture,
    C: ResourceFuture,
    D: ResourceFuture,
    E: ResourceFuture,
    F: ResourceFuture,
    G: ResourceFuture,
    H: ResourceFuture,
{
    type Body = Either8<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body>;

    fn poll_response(&mut self, request: &http::Request<()>) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either8::*;

        let response = match *self {
            A(ref mut f) => try_ready!(f.poll_response(request)).map(A),
            B(ref mut f) => try_ready!(f.poll_response(request)).map(B),
            C(ref mut f) => try_ready!(f.poll_response(request)).map(C),
            D(ref mut f) => try_ready!(f.poll_response(request)).map(D),
            E(ref mut f) => try_ready!(f.poll_response(request)).map(E),
            F(ref mut f) => try_ready!(f.poll_response(request)).map(F),
            G(ref mut f) => try_ready!(f.poll_response(request)).map(G),
            H(ref mut f) => try_ready!(f.poll_response(request)).map(H),
        };
        Ok(response.into())
    }
}

impl<A, B, C, D, E, F, G, H> Either8<A, B, C, D, E, F, G, H>
where
    A: ExtractFuture,
    B: ExtractFuture,
    C: ExtractFuture,
    D: ExtractFuture,
    E: ExtractFuture,
    F: ExtractFuture,
    G: ExtractFuture,
    H: ExtractFuture,
{

    pub fn poll_ready(&mut self) -> Poll<(), extract::Error> {
        use self::Either8::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
            C(ref mut f) => f.poll(),
            D(ref mut f) => f.poll(),
            E(ref mut f) => f.poll(),
            F(ref mut f) => f.poll(),
            G(ref mut f) => f.poll(),
            H(ref mut f) => f.poll(),
        }
    }
}

impl<A, B, C, D, E, F, G, H> HttpFuture for Either8<A, B, C, D, E, F, G, H>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
    G: HttpFuture,
    H: HttpFuture,
{
    type Body = Either8<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body>;

    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either8::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll_http()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll_http()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll_http()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll_http()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll_http()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll_http()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll_http()).map(H).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H> SealedFuture for Either8<A, B, C, D, E, F, G, H>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
    G: HttpFuture,
    H: HttpFuture,
{
}

impl<A, B, C, D, E, F, G, H> Stream for Either8<A, B, C, D, E, F, G, H>
where
    A: Stream,
    B: Stream<Error = A::Error>,
    C: Stream<Error = A::Error>,
    D: Stream<Error = A::Error>,
    E: Stream<Error = A::Error>,
    F: Stream<Error = A::Error>,
    G: Stream<Error = A::Error>,
    H: Stream<Error = A::Error>,
{
    type Item = Either8<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either8::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll()).map(H).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H> BufStream for Either8<A, B, C, D, E, F, G, H>
where
    A: BufStream,
    B: BufStream<Error = A::Error>,
    C: BufStream<Error = A::Error>,
    D: BufStream<Error = A::Error>,
    E: BufStream<Error = A::Error>,
    F: BufStream<Error = A::Error>,
    G: BufStream<Error = A::Error>,
    H: BufStream<Error = A::Error>,
{
    type Item = Either8<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either8::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll()).map(H).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H> Buf for Either8<A, B, C, D, E, F, G, H>
where
    A: Buf,
    B: Buf,
    C: Buf,
    D: Buf,
    E: Buf,
    F: Buf,
    G: Buf,
    H: Buf,
{
    fn remaining(&self) -> usize {
        use self::Either8::*;

        match *self {
            A(ref b) => b.remaining(),
            B(ref b) => b.remaining(),
            C(ref b) => b.remaining(),
            D(ref b) => b.remaining(),
            E(ref b) => b.remaining(),
            F(ref b) => b.remaining(),
            G(ref b) => b.remaining(),
            H(ref b) => b.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        use self::Either8::*;

        match *self {
            A(ref b) => b.bytes(),
            B(ref b) => b.bytes(),
            C(ref b) => b.bytes(),
            D(ref b) => b.bytes(),
            E(ref b) => b.bytes(),
            F(ref b) => b.bytes(),
            G(ref b) => b.bytes(),
            H(ref b) => b.bytes(),
        }
    }

    fn advance(&mut self, cnt: usize) {
        use self::Either8::*;

        match *self {
            A(ref mut b) => b.advance(cnt),
            B(ref mut b) => b.advance(cnt),
            C(ref mut b) => b.advance(cnt),
            D(ref mut b) => b.advance(cnt),
            E(ref mut b) => b.advance(cnt),
            F(ref mut b) => b.advance(cnt),
            G(ref mut b) => b.advance(cnt),
            H(ref mut b) => b.advance(cnt),
        }
    }
}

impl<A, B, C, D, E, F, G, H> Response for Either8<A, B, C, D, E, F, G, H>
where
    A: Response,
    B: Response,
    C: Response,
    D: Response,
    E: Response,
    F: Response,
    G: Response,
    H: Response,
{
    type Buf = Either8<A::Buf, B::Buf, C::Buf, D::Buf, E::Buf, F::Buf, G::Buf, H::Buf>;
    type Body = Either8<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where S: Serializer
    {
        use self::Either8::*;

        match self {
            A(r) => Ok(r.into_http(context)?.map(Either8::A)),
            B(r) => Ok(r.into_http(context)?.map(Either8::B)),
            C(r) => Ok(r.into_http(context)?.map(Either8::C)),
            D(r) => Ok(r.into_http(context)?.map(Either8::D)),
            E(r) => Ok(r.into_http(context)?.map(Either8::E)),
            F(r) => Ok(r.into_http(context)?.map(Either8::F)),
            G(r) => Ok(r.into_http(context)?.map(Either8::G)),
            H(r) => Ok(r.into_http(context)?.map(Either8::H)),
        }
    }
}

impl<R0, R1, R2, R3, R4, R5, R6, R7> Resource for (R0, R1, R2, R3, R4, R5, R6, R7,)
where
    R0: Resource,
    R1: Resource<RequestBody = R0::RequestBody>,
    R2: Resource<RequestBody = R0::RequestBody>,
    R3: Resource<RequestBody = R0::RequestBody>,
    R4: Resource<RequestBody = R0::RequestBody>,
    R5: Resource<RequestBody = R0::RequestBody>,
    R6: Resource<RequestBody = R0::RequestBody>,
    R7: Resource<RequestBody = R0::RequestBody>,
{
    type Destination = Either8<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination, R5::Destination, R6::Destination, R7::Destination>;
    type RequestBody = R0::RequestBody;
    type Buf = Either8<R0::Buf, R1::Buf, R2::Buf, R3::Buf, R4::Buf, R5::Buf, R6::Buf, R7::Buf>;
    type Body = Either8<R0::Body, R1::Body, R2::Body, R3::Body, R4::Body, R5::Body, R6::Body, R7::Body>;
    type Future = Either8<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future, R5::Future, R6::Future, R7::Future>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: &RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either8::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, body))
            }
            C(d) => {
                C(self.2.dispatch(d, route_match, body))
            }
            D(d) => {
                D(self.3.dispatch(d, route_match, body))
            }
            E(d) => {
                E(self.4.dispatch(d, route_match, body))
            }
            F(d) => {
                F(self.5.dispatch(d, route_match, body))
            }
            G(d) => {
                G(self.6.dispatch(d, route_match, body))
            }
            H(d) => {
                H(self.7.dispatch(d, route_match, body))
            }
        }
    }
}
// ===== 9 =====

#[derive(Debug, Clone)]
pub enum Either9<A, B, C, D, E, F, G, H, I> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
    H(H),
    I(I),
}

impl<A, B, C, D, E, F, G, H, I> Future for Either9<A, B, C, D, E, F, G, H, I>
where
    A: Future,
    B: Future<Error = A::Error>,
    C: Future<Error = A::Error>,
    D: Future<Error = A::Error>,
    E: Future<Error = A::Error>,
    F: Future<Error = A::Error>,
    G: Future<Error = A::Error>,
    H: Future<Error = A::Error>,
    I: Future<Error = A::Error>,
{
    type Item = Either9<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item, I::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either9::*;

        match *self {
            A(ref mut f) => Ok(A(try_ready!(f.poll())).into()),
            B(ref mut f) => Ok(B(try_ready!(f.poll())).into()),
            C(ref mut f) => Ok(C(try_ready!(f.poll())).into()),
            D(ref mut f) => Ok(D(try_ready!(f.poll())).into()),
            E(ref mut f) => Ok(E(try_ready!(f.poll())).into()),
            F(ref mut f) => Ok(F(try_ready!(f.poll())).into()),
            G(ref mut f) => Ok(G(try_ready!(f.poll())).into()),
            H(ref mut f) => Ok(H(try_ready!(f.poll())).into()),
            I(ref mut f) => Ok(I(try_ready!(f.poll())).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I> ResourceFuture for Either9<A, B, C, D, E, F, G, H, I>
where
    A: ResourceFuture,
    B: ResourceFuture,
    C: ResourceFuture,
    D: ResourceFuture,
    E: ResourceFuture,
    F: ResourceFuture,
    G: ResourceFuture,
    H: ResourceFuture,
    I: ResourceFuture,
{
    type Body = Either9<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body, I::Body>;

    fn poll_response(&mut self, request: &http::Request<()>) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either9::*;

        let response = match *self {
            A(ref mut f) => try_ready!(f.poll_response(request)).map(A),
            B(ref mut f) => try_ready!(f.poll_response(request)).map(B),
            C(ref mut f) => try_ready!(f.poll_response(request)).map(C),
            D(ref mut f) => try_ready!(f.poll_response(request)).map(D),
            E(ref mut f) => try_ready!(f.poll_response(request)).map(E),
            F(ref mut f) => try_ready!(f.poll_response(request)).map(F),
            G(ref mut f) => try_ready!(f.poll_response(request)).map(G),
            H(ref mut f) => try_ready!(f.poll_response(request)).map(H),
            I(ref mut f) => try_ready!(f.poll_response(request)).map(I),
        };
        Ok(response.into())
    }
}

impl<A, B, C, D, E, F, G, H, I> Either9<A, B, C, D, E, F, G, H, I>
where
    A: ExtractFuture,
    B: ExtractFuture,
    C: ExtractFuture,
    D: ExtractFuture,
    E: ExtractFuture,
    F: ExtractFuture,
    G: ExtractFuture,
    H: ExtractFuture,
    I: ExtractFuture,
{

    pub fn poll_ready(&mut self) -> Poll<(), extract::Error> {
        use self::Either9::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
            C(ref mut f) => f.poll(),
            D(ref mut f) => f.poll(),
            E(ref mut f) => f.poll(),
            F(ref mut f) => f.poll(),
            G(ref mut f) => f.poll(),
            H(ref mut f) => f.poll(),
            I(ref mut f) => f.poll(),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I> HttpFuture for Either9<A, B, C, D, E, F, G, H, I>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
    G: HttpFuture,
    H: HttpFuture,
    I: HttpFuture,
{
    type Body = Either9<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body, I::Body>;

    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either9::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll_http()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll_http()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll_http()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll_http()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll_http()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll_http()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll_http()).map(H).into()),
            I(ref mut f) => Ok(try_ready!(f.poll_http()).map(I).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I> SealedFuture for Either9<A, B, C, D, E, F, G, H, I>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
    G: HttpFuture,
    H: HttpFuture,
    I: HttpFuture,
{
}

impl<A, B, C, D, E, F, G, H, I> Stream for Either9<A, B, C, D, E, F, G, H, I>
where
    A: Stream,
    B: Stream<Error = A::Error>,
    C: Stream<Error = A::Error>,
    D: Stream<Error = A::Error>,
    E: Stream<Error = A::Error>,
    F: Stream<Error = A::Error>,
    G: Stream<Error = A::Error>,
    H: Stream<Error = A::Error>,
    I: Stream<Error = A::Error>,
{
    type Item = Either9<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item, I::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either9::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll()).map(H).into()),
            I(ref mut f) => Ok(try_ready!(f.poll()).map(I).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I> BufStream for Either9<A, B, C, D, E, F, G, H, I>
where
    A: BufStream,
    B: BufStream<Error = A::Error>,
    C: BufStream<Error = A::Error>,
    D: BufStream<Error = A::Error>,
    E: BufStream<Error = A::Error>,
    F: BufStream<Error = A::Error>,
    G: BufStream<Error = A::Error>,
    H: BufStream<Error = A::Error>,
    I: BufStream<Error = A::Error>,
{
    type Item = Either9<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item, I::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either9::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll()).map(H).into()),
            I(ref mut f) => Ok(try_ready!(f.poll()).map(I).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I> Buf for Either9<A, B, C, D, E, F, G, H, I>
where
    A: Buf,
    B: Buf,
    C: Buf,
    D: Buf,
    E: Buf,
    F: Buf,
    G: Buf,
    H: Buf,
    I: Buf,
{
    fn remaining(&self) -> usize {
        use self::Either9::*;

        match *self {
            A(ref b) => b.remaining(),
            B(ref b) => b.remaining(),
            C(ref b) => b.remaining(),
            D(ref b) => b.remaining(),
            E(ref b) => b.remaining(),
            F(ref b) => b.remaining(),
            G(ref b) => b.remaining(),
            H(ref b) => b.remaining(),
            I(ref b) => b.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        use self::Either9::*;

        match *self {
            A(ref b) => b.bytes(),
            B(ref b) => b.bytes(),
            C(ref b) => b.bytes(),
            D(ref b) => b.bytes(),
            E(ref b) => b.bytes(),
            F(ref b) => b.bytes(),
            G(ref b) => b.bytes(),
            H(ref b) => b.bytes(),
            I(ref b) => b.bytes(),
        }
    }

    fn advance(&mut self, cnt: usize) {
        use self::Either9::*;

        match *self {
            A(ref mut b) => b.advance(cnt),
            B(ref mut b) => b.advance(cnt),
            C(ref mut b) => b.advance(cnt),
            D(ref mut b) => b.advance(cnt),
            E(ref mut b) => b.advance(cnt),
            F(ref mut b) => b.advance(cnt),
            G(ref mut b) => b.advance(cnt),
            H(ref mut b) => b.advance(cnt),
            I(ref mut b) => b.advance(cnt),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I> Response for Either9<A, B, C, D, E, F, G, H, I>
where
    A: Response,
    B: Response,
    C: Response,
    D: Response,
    E: Response,
    F: Response,
    G: Response,
    H: Response,
    I: Response,
{
    type Buf = Either9<A::Buf, B::Buf, C::Buf, D::Buf, E::Buf, F::Buf, G::Buf, H::Buf, I::Buf>;
    type Body = Either9<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body, I::Body>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where S: Serializer
    {
        use self::Either9::*;

        match self {
            A(r) => Ok(r.into_http(context)?.map(Either9::A)),
            B(r) => Ok(r.into_http(context)?.map(Either9::B)),
            C(r) => Ok(r.into_http(context)?.map(Either9::C)),
            D(r) => Ok(r.into_http(context)?.map(Either9::D)),
            E(r) => Ok(r.into_http(context)?.map(Either9::E)),
            F(r) => Ok(r.into_http(context)?.map(Either9::F)),
            G(r) => Ok(r.into_http(context)?.map(Either9::G)),
            H(r) => Ok(r.into_http(context)?.map(Either9::H)),
            I(r) => Ok(r.into_http(context)?.map(Either9::I)),
        }
    }
}

impl<R0, R1, R2, R3, R4, R5, R6, R7, R8> Resource for (R0, R1, R2, R3, R4, R5, R6, R7, R8,)
where
    R0: Resource,
    R1: Resource<RequestBody = R0::RequestBody>,
    R2: Resource<RequestBody = R0::RequestBody>,
    R3: Resource<RequestBody = R0::RequestBody>,
    R4: Resource<RequestBody = R0::RequestBody>,
    R5: Resource<RequestBody = R0::RequestBody>,
    R6: Resource<RequestBody = R0::RequestBody>,
    R7: Resource<RequestBody = R0::RequestBody>,
    R8: Resource<RequestBody = R0::RequestBody>,
{
    type Destination = Either9<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination, R5::Destination, R6::Destination, R7::Destination, R8::Destination>;
    type RequestBody = R0::RequestBody;
    type Buf = Either9<R0::Buf, R1::Buf, R2::Buf, R3::Buf, R4::Buf, R5::Buf, R6::Buf, R7::Buf, R8::Buf>;
    type Body = Either9<R0::Body, R1::Body, R2::Body, R3::Body, R4::Body, R5::Body, R6::Body, R7::Body, R8::Body>;
    type Future = Either9<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future, R5::Future, R6::Future, R7::Future, R8::Future>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: &RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either9::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, body))
            }
            C(d) => {
                C(self.2.dispatch(d, route_match, body))
            }
            D(d) => {
                D(self.3.dispatch(d, route_match, body))
            }
            E(d) => {
                E(self.4.dispatch(d, route_match, body))
            }
            F(d) => {
                F(self.5.dispatch(d, route_match, body))
            }
            G(d) => {
                G(self.6.dispatch(d, route_match, body))
            }
            H(d) => {
                H(self.7.dispatch(d, route_match, body))
            }
            I(d) => {
                I(self.8.dispatch(d, route_match, body))
            }
        }
    }
}
// ===== 10 =====

#[derive(Debug, Clone)]
pub enum Either10<A, B, C, D, E, F, G, H, I, J> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
    H(H),
    I(I),
    J(J),
}

impl<A, B, C, D, E, F, G, H, I, J> Future for Either10<A, B, C, D, E, F, G, H, I, J>
where
    A: Future,
    B: Future<Error = A::Error>,
    C: Future<Error = A::Error>,
    D: Future<Error = A::Error>,
    E: Future<Error = A::Error>,
    F: Future<Error = A::Error>,
    G: Future<Error = A::Error>,
    H: Future<Error = A::Error>,
    I: Future<Error = A::Error>,
    J: Future<Error = A::Error>,
{
    type Item = Either10<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item, I::Item, J::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either10::*;

        match *self {
            A(ref mut f) => Ok(A(try_ready!(f.poll())).into()),
            B(ref mut f) => Ok(B(try_ready!(f.poll())).into()),
            C(ref mut f) => Ok(C(try_ready!(f.poll())).into()),
            D(ref mut f) => Ok(D(try_ready!(f.poll())).into()),
            E(ref mut f) => Ok(E(try_ready!(f.poll())).into()),
            F(ref mut f) => Ok(F(try_ready!(f.poll())).into()),
            G(ref mut f) => Ok(G(try_ready!(f.poll())).into()),
            H(ref mut f) => Ok(H(try_ready!(f.poll())).into()),
            I(ref mut f) => Ok(I(try_ready!(f.poll())).into()),
            J(ref mut f) => Ok(J(try_ready!(f.poll())).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J> ResourceFuture for Either10<A, B, C, D, E, F, G, H, I, J>
where
    A: ResourceFuture,
    B: ResourceFuture,
    C: ResourceFuture,
    D: ResourceFuture,
    E: ResourceFuture,
    F: ResourceFuture,
    G: ResourceFuture,
    H: ResourceFuture,
    I: ResourceFuture,
    J: ResourceFuture,
{
    type Body = Either10<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body, I::Body, J::Body>;

    fn poll_response(&mut self, request: &http::Request<()>) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either10::*;

        let response = match *self {
            A(ref mut f) => try_ready!(f.poll_response(request)).map(A),
            B(ref mut f) => try_ready!(f.poll_response(request)).map(B),
            C(ref mut f) => try_ready!(f.poll_response(request)).map(C),
            D(ref mut f) => try_ready!(f.poll_response(request)).map(D),
            E(ref mut f) => try_ready!(f.poll_response(request)).map(E),
            F(ref mut f) => try_ready!(f.poll_response(request)).map(F),
            G(ref mut f) => try_ready!(f.poll_response(request)).map(G),
            H(ref mut f) => try_ready!(f.poll_response(request)).map(H),
            I(ref mut f) => try_ready!(f.poll_response(request)).map(I),
            J(ref mut f) => try_ready!(f.poll_response(request)).map(J),
        };
        Ok(response.into())
    }
}

impl<A, B, C, D, E, F, G, H, I, J> Either10<A, B, C, D, E, F, G, H, I, J>
where
    A: ExtractFuture,
    B: ExtractFuture,
    C: ExtractFuture,
    D: ExtractFuture,
    E: ExtractFuture,
    F: ExtractFuture,
    G: ExtractFuture,
    H: ExtractFuture,
    I: ExtractFuture,
    J: ExtractFuture,
{

    pub fn poll_ready(&mut self) -> Poll<(), extract::Error> {
        use self::Either10::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
            C(ref mut f) => f.poll(),
            D(ref mut f) => f.poll(),
            E(ref mut f) => f.poll(),
            F(ref mut f) => f.poll(),
            G(ref mut f) => f.poll(),
            H(ref mut f) => f.poll(),
            I(ref mut f) => f.poll(),
            J(ref mut f) => f.poll(),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J> HttpFuture for Either10<A, B, C, D, E, F, G, H, I, J>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
    G: HttpFuture,
    H: HttpFuture,
    I: HttpFuture,
    J: HttpFuture,
{
    type Body = Either10<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body, I::Body, J::Body>;

    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either10::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll_http()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll_http()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll_http()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll_http()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll_http()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll_http()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll_http()).map(H).into()),
            I(ref mut f) => Ok(try_ready!(f.poll_http()).map(I).into()),
            J(ref mut f) => Ok(try_ready!(f.poll_http()).map(J).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J> SealedFuture for Either10<A, B, C, D, E, F, G, H, I, J>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
    G: HttpFuture,
    H: HttpFuture,
    I: HttpFuture,
    J: HttpFuture,
{
}

impl<A, B, C, D, E, F, G, H, I, J> Stream for Either10<A, B, C, D, E, F, G, H, I, J>
where
    A: Stream,
    B: Stream<Error = A::Error>,
    C: Stream<Error = A::Error>,
    D: Stream<Error = A::Error>,
    E: Stream<Error = A::Error>,
    F: Stream<Error = A::Error>,
    G: Stream<Error = A::Error>,
    H: Stream<Error = A::Error>,
    I: Stream<Error = A::Error>,
    J: Stream<Error = A::Error>,
{
    type Item = Either10<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item, I::Item, J::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either10::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll()).map(H).into()),
            I(ref mut f) => Ok(try_ready!(f.poll()).map(I).into()),
            J(ref mut f) => Ok(try_ready!(f.poll()).map(J).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J> BufStream for Either10<A, B, C, D, E, F, G, H, I, J>
where
    A: BufStream,
    B: BufStream<Error = A::Error>,
    C: BufStream<Error = A::Error>,
    D: BufStream<Error = A::Error>,
    E: BufStream<Error = A::Error>,
    F: BufStream<Error = A::Error>,
    G: BufStream<Error = A::Error>,
    H: BufStream<Error = A::Error>,
    I: BufStream<Error = A::Error>,
    J: BufStream<Error = A::Error>,
{
    type Item = Either10<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item, I::Item, J::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either10::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll()).map(H).into()),
            I(ref mut f) => Ok(try_ready!(f.poll()).map(I).into()),
            J(ref mut f) => Ok(try_ready!(f.poll()).map(J).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J> Buf for Either10<A, B, C, D, E, F, G, H, I, J>
where
    A: Buf,
    B: Buf,
    C: Buf,
    D: Buf,
    E: Buf,
    F: Buf,
    G: Buf,
    H: Buf,
    I: Buf,
    J: Buf,
{
    fn remaining(&self) -> usize {
        use self::Either10::*;

        match *self {
            A(ref b) => b.remaining(),
            B(ref b) => b.remaining(),
            C(ref b) => b.remaining(),
            D(ref b) => b.remaining(),
            E(ref b) => b.remaining(),
            F(ref b) => b.remaining(),
            G(ref b) => b.remaining(),
            H(ref b) => b.remaining(),
            I(ref b) => b.remaining(),
            J(ref b) => b.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        use self::Either10::*;

        match *self {
            A(ref b) => b.bytes(),
            B(ref b) => b.bytes(),
            C(ref b) => b.bytes(),
            D(ref b) => b.bytes(),
            E(ref b) => b.bytes(),
            F(ref b) => b.bytes(),
            G(ref b) => b.bytes(),
            H(ref b) => b.bytes(),
            I(ref b) => b.bytes(),
            J(ref b) => b.bytes(),
        }
    }

    fn advance(&mut self, cnt: usize) {
        use self::Either10::*;

        match *self {
            A(ref mut b) => b.advance(cnt),
            B(ref mut b) => b.advance(cnt),
            C(ref mut b) => b.advance(cnt),
            D(ref mut b) => b.advance(cnt),
            E(ref mut b) => b.advance(cnt),
            F(ref mut b) => b.advance(cnt),
            G(ref mut b) => b.advance(cnt),
            H(ref mut b) => b.advance(cnt),
            I(ref mut b) => b.advance(cnt),
            J(ref mut b) => b.advance(cnt),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J> Response for Either10<A, B, C, D, E, F, G, H, I, J>
where
    A: Response,
    B: Response,
    C: Response,
    D: Response,
    E: Response,
    F: Response,
    G: Response,
    H: Response,
    I: Response,
    J: Response,
{
    type Buf = Either10<A::Buf, B::Buf, C::Buf, D::Buf, E::Buf, F::Buf, G::Buf, H::Buf, I::Buf, J::Buf>;
    type Body = Either10<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body, I::Body, J::Body>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where S: Serializer
    {
        use self::Either10::*;

        match self {
            A(r) => Ok(r.into_http(context)?.map(Either10::A)),
            B(r) => Ok(r.into_http(context)?.map(Either10::B)),
            C(r) => Ok(r.into_http(context)?.map(Either10::C)),
            D(r) => Ok(r.into_http(context)?.map(Either10::D)),
            E(r) => Ok(r.into_http(context)?.map(Either10::E)),
            F(r) => Ok(r.into_http(context)?.map(Either10::F)),
            G(r) => Ok(r.into_http(context)?.map(Either10::G)),
            H(r) => Ok(r.into_http(context)?.map(Either10::H)),
            I(r) => Ok(r.into_http(context)?.map(Either10::I)),
            J(r) => Ok(r.into_http(context)?.map(Either10::J)),
        }
    }
}

impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9> Resource for (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9,)
where
    R0: Resource,
    R1: Resource<RequestBody = R0::RequestBody>,
    R2: Resource<RequestBody = R0::RequestBody>,
    R3: Resource<RequestBody = R0::RequestBody>,
    R4: Resource<RequestBody = R0::RequestBody>,
    R5: Resource<RequestBody = R0::RequestBody>,
    R6: Resource<RequestBody = R0::RequestBody>,
    R7: Resource<RequestBody = R0::RequestBody>,
    R8: Resource<RequestBody = R0::RequestBody>,
    R9: Resource<RequestBody = R0::RequestBody>,
{
    type Destination = Either10<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination, R5::Destination, R6::Destination, R7::Destination, R8::Destination, R9::Destination>;
    type RequestBody = R0::RequestBody;
    type Buf = Either10<R0::Buf, R1::Buf, R2::Buf, R3::Buf, R4::Buf, R5::Buf, R6::Buf, R7::Buf, R8::Buf, R9::Buf>;
    type Body = Either10<R0::Body, R1::Body, R2::Body, R3::Body, R4::Body, R5::Body, R6::Body, R7::Body, R8::Body, R9::Body>;
    type Future = Either10<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future, R5::Future, R6::Future, R7::Future, R8::Future, R9::Future>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: &RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either10::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, body))
            }
            C(d) => {
                C(self.2.dispatch(d, route_match, body))
            }
            D(d) => {
                D(self.3.dispatch(d, route_match, body))
            }
            E(d) => {
                E(self.4.dispatch(d, route_match, body))
            }
            F(d) => {
                F(self.5.dispatch(d, route_match, body))
            }
            G(d) => {
                G(self.6.dispatch(d, route_match, body))
            }
            H(d) => {
                H(self.7.dispatch(d, route_match, body))
            }
            I(d) => {
                I(self.8.dispatch(d, route_match, body))
            }
            J(d) => {
                J(self.9.dispatch(d, route_match, body))
            }
        }
    }
}
// ===== 11 =====

#[derive(Debug, Clone)]
pub enum Either11<A, B, C, D, E, F, G, H, I, J, K> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
    H(H),
    I(I),
    J(J),
    K(K),
}

impl<A, B, C, D, E, F, G, H, I, J, K> Future for Either11<A, B, C, D, E, F, G, H, I, J, K>
where
    A: Future,
    B: Future<Error = A::Error>,
    C: Future<Error = A::Error>,
    D: Future<Error = A::Error>,
    E: Future<Error = A::Error>,
    F: Future<Error = A::Error>,
    G: Future<Error = A::Error>,
    H: Future<Error = A::Error>,
    I: Future<Error = A::Error>,
    J: Future<Error = A::Error>,
    K: Future<Error = A::Error>,
{
    type Item = Either11<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item, I::Item, J::Item, K::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either11::*;

        match *self {
            A(ref mut f) => Ok(A(try_ready!(f.poll())).into()),
            B(ref mut f) => Ok(B(try_ready!(f.poll())).into()),
            C(ref mut f) => Ok(C(try_ready!(f.poll())).into()),
            D(ref mut f) => Ok(D(try_ready!(f.poll())).into()),
            E(ref mut f) => Ok(E(try_ready!(f.poll())).into()),
            F(ref mut f) => Ok(F(try_ready!(f.poll())).into()),
            G(ref mut f) => Ok(G(try_ready!(f.poll())).into()),
            H(ref mut f) => Ok(H(try_ready!(f.poll())).into()),
            I(ref mut f) => Ok(I(try_ready!(f.poll())).into()),
            J(ref mut f) => Ok(J(try_ready!(f.poll())).into()),
            K(ref mut f) => Ok(K(try_ready!(f.poll())).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> ResourceFuture for Either11<A, B, C, D, E, F, G, H, I, J, K>
where
    A: ResourceFuture,
    B: ResourceFuture,
    C: ResourceFuture,
    D: ResourceFuture,
    E: ResourceFuture,
    F: ResourceFuture,
    G: ResourceFuture,
    H: ResourceFuture,
    I: ResourceFuture,
    J: ResourceFuture,
    K: ResourceFuture,
{
    type Body = Either11<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body, I::Body, J::Body, K::Body>;

    fn poll_response(&mut self, request: &http::Request<()>) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either11::*;

        let response = match *self {
            A(ref mut f) => try_ready!(f.poll_response(request)).map(A),
            B(ref mut f) => try_ready!(f.poll_response(request)).map(B),
            C(ref mut f) => try_ready!(f.poll_response(request)).map(C),
            D(ref mut f) => try_ready!(f.poll_response(request)).map(D),
            E(ref mut f) => try_ready!(f.poll_response(request)).map(E),
            F(ref mut f) => try_ready!(f.poll_response(request)).map(F),
            G(ref mut f) => try_ready!(f.poll_response(request)).map(G),
            H(ref mut f) => try_ready!(f.poll_response(request)).map(H),
            I(ref mut f) => try_ready!(f.poll_response(request)).map(I),
            J(ref mut f) => try_ready!(f.poll_response(request)).map(J),
            K(ref mut f) => try_ready!(f.poll_response(request)).map(K),
        };
        Ok(response.into())
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> Either11<A, B, C, D, E, F, G, H, I, J, K>
where
    A: ExtractFuture,
    B: ExtractFuture,
    C: ExtractFuture,
    D: ExtractFuture,
    E: ExtractFuture,
    F: ExtractFuture,
    G: ExtractFuture,
    H: ExtractFuture,
    I: ExtractFuture,
    J: ExtractFuture,
    K: ExtractFuture,
{

    pub fn poll_ready(&mut self) -> Poll<(), extract::Error> {
        use self::Either11::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
            C(ref mut f) => f.poll(),
            D(ref mut f) => f.poll(),
            E(ref mut f) => f.poll(),
            F(ref mut f) => f.poll(),
            G(ref mut f) => f.poll(),
            H(ref mut f) => f.poll(),
            I(ref mut f) => f.poll(),
            J(ref mut f) => f.poll(),
            K(ref mut f) => f.poll(),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> HttpFuture for Either11<A, B, C, D, E, F, G, H, I, J, K>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
    G: HttpFuture,
    H: HttpFuture,
    I: HttpFuture,
    J: HttpFuture,
    K: HttpFuture,
{
    type Body = Either11<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body, I::Body, J::Body, K::Body>;

    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either11::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll_http()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll_http()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll_http()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll_http()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll_http()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll_http()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll_http()).map(H).into()),
            I(ref mut f) => Ok(try_ready!(f.poll_http()).map(I).into()),
            J(ref mut f) => Ok(try_ready!(f.poll_http()).map(J).into()),
            K(ref mut f) => Ok(try_ready!(f.poll_http()).map(K).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> SealedFuture for Either11<A, B, C, D, E, F, G, H, I, J, K>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
    G: HttpFuture,
    H: HttpFuture,
    I: HttpFuture,
    J: HttpFuture,
    K: HttpFuture,
{
}

impl<A, B, C, D, E, F, G, H, I, J, K> Stream for Either11<A, B, C, D, E, F, G, H, I, J, K>
where
    A: Stream,
    B: Stream<Error = A::Error>,
    C: Stream<Error = A::Error>,
    D: Stream<Error = A::Error>,
    E: Stream<Error = A::Error>,
    F: Stream<Error = A::Error>,
    G: Stream<Error = A::Error>,
    H: Stream<Error = A::Error>,
    I: Stream<Error = A::Error>,
    J: Stream<Error = A::Error>,
    K: Stream<Error = A::Error>,
{
    type Item = Either11<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item, I::Item, J::Item, K::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either11::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll()).map(H).into()),
            I(ref mut f) => Ok(try_ready!(f.poll()).map(I).into()),
            J(ref mut f) => Ok(try_ready!(f.poll()).map(J).into()),
            K(ref mut f) => Ok(try_ready!(f.poll()).map(K).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> BufStream for Either11<A, B, C, D, E, F, G, H, I, J, K>
where
    A: BufStream,
    B: BufStream<Error = A::Error>,
    C: BufStream<Error = A::Error>,
    D: BufStream<Error = A::Error>,
    E: BufStream<Error = A::Error>,
    F: BufStream<Error = A::Error>,
    G: BufStream<Error = A::Error>,
    H: BufStream<Error = A::Error>,
    I: BufStream<Error = A::Error>,
    J: BufStream<Error = A::Error>,
    K: BufStream<Error = A::Error>,
{
    type Item = Either11<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item, I::Item, J::Item, K::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either11::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll()).map(H).into()),
            I(ref mut f) => Ok(try_ready!(f.poll()).map(I).into()),
            J(ref mut f) => Ok(try_ready!(f.poll()).map(J).into()),
            K(ref mut f) => Ok(try_ready!(f.poll()).map(K).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> Buf for Either11<A, B, C, D, E, F, G, H, I, J, K>
where
    A: Buf,
    B: Buf,
    C: Buf,
    D: Buf,
    E: Buf,
    F: Buf,
    G: Buf,
    H: Buf,
    I: Buf,
    J: Buf,
    K: Buf,
{
    fn remaining(&self) -> usize {
        use self::Either11::*;

        match *self {
            A(ref b) => b.remaining(),
            B(ref b) => b.remaining(),
            C(ref b) => b.remaining(),
            D(ref b) => b.remaining(),
            E(ref b) => b.remaining(),
            F(ref b) => b.remaining(),
            G(ref b) => b.remaining(),
            H(ref b) => b.remaining(),
            I(ref b) => b.remaining(),
            J(ref b) => b.remaining(),
            K(ref b) => b.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        use self::Either11::*;

        match *self {
            A(ref b) => b.bytes(),
            B(ref b) => b.bytes(),
            C(ref b) => b.bytes(),
            D(ref b) => b.bytes(),
            E(ref b) => b.bytes(),
            F(ref b) => b.bytes(),
            G(ref b) => b.bytes(),
            H(ref b) => b.bytes(),
            I(ref b) => b.bytes(),
            J(ref b) => b.bytes(),
            K(ref b) => b.bytes(),
        }
    }

    fn advance(&mut self, cnt: usize) {
        use self::Either11::*;

        match *self {
            A(ref mut b) => b.advance(cnt),
            B(ref mut b) => b.advance(cnt),
            C(ref mut b) => b.advance(cnt),
            D(ref mut b) => b.advance(cnt),
            E(ref mut b) => b.advance(cnt),
            F(ref mut b) => b.advance(cnt),
            G(ref mut b) => b.advance(cnt),
            H(ref mut b) => b.advance(cnt),
            I(ref mut b) => b.advance(cnt),
            J(ref mut b) => b.advance(cnt),
            K(ref mut b) => b.advance(cnt),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> Response for Either11<A, B, C, D, E, F, G, H, I, J, K>
where
    A: Response,
    B: Response,
    C: Response,
    D: Response,
    E: Response,
    F: Response,
    G: Response,
    H: Response,
    I: Response,
    J: Response,
    K: Response,
{
    type Buf = Either11<A::Buf, B::Buf, C::Buf, D::Buf, E::Buf, F::Buf, G::Buf, H::Buf, I::Buf, J::Buf, K::Buf>;
    type Body = Either11<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body, I::Body, J::Body, K::Body>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where S: Serializer
    {
        use self::Either11::*;

        match self {
            A(r) => Ok(r.into_http(context)?.map(Either11::A)),
            B(r) => Ok(r.into_http(context)?.map(Either11::B)),
            C(r) => Ok(r.into_http(context)?.map(Either11::C)),
            D(r) => Ok(r.into_http(context)?.map(Either11::D)),
            E(r) => Ok(r.into_http(context)?.map(Either11::E)),
            F(r) => Ok(r.into_http(context)?.map(Either11::F)),
            G(r) => Ok(r.into_http(context)?.map(Either11::G)),
            H(r) => Ok(r.into_http(context)?.map(Either11::H)),
            I(r) => Ok(r.into_http(context)?.map(Either11::I)),
            J(r) => Ok(r.into_http(context)?.map(Either11::J)),
            K(r) => Ok(r.into_http(context)?.map(Either11::K)),
        }
    }
}

impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10> Resource for (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10,)
where
    R0: Resource,
    R1: Resource<RequestBody = R0::RequestBody>,
    R2: Resource<RequestBody = R0::RequestBody>,
    R3: Resource<RequestBody = R0::RequestBody>,
    R4: Resource<RequestBody = R0::RequestBody>,
    R5: Resource<RequestBody = R0::RequestBody>,
    R6: Resource<RequestBody = R0::RequestBody>,
    R7: Resource<RequestBody = R0::RequestBody>,
    R8: Resource<RequestBody = R0::RequestBody>,
    R9: Resource<RequestBody = R0::RequestBody>,
    R10: Resource<RequestBody = R0::RequestBody>,
{
    type Destination = Either11<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination, R5::Destination, R6::Destination, R7::Destination, R8::Destination, R9::Destination, R10::Destination>;
    type RequestBody = R0::RequestBody;
    type Buf = Either11<R0::Buf, R1::Buf, R2::Buf, R3::Buf, R4::Buf, R5::Buf, R6::Buf, R7::Buf, R8::Buf, R9::Buf, R10::Buf>;
    type Body = Either11<R0::Body, R1::Body, R2::Body, R3::Body, R4::Body, R5::Body, R6::Body, R7::Body, R8::Body, R9::Body, R10::Body>;
    type Future = Either11<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future, R5::Future, R6::Future, R7::Future, R8::Future, R9::Future, R10::Future>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: &RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either11::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, body))
            }
            C(d) => {
                C(self.2.dispatch(d, route_match, body))
            }
            D(d) => {
                D(self.3.dispatch(d, route_match, body))
            }
            E(d) => {
                E(self.4.dispatch(d, route_match, body))
            }
            F(d) => {
                F(self.5.dispatch(d, route_match, body))
            }
            G(d) => {
                G(self.6.dispatch(d, route_match, body))
            }
            H(d) => {
                H(self.7.dispatch(d, route_match, body))
            }
            I(d) => {
                I(self.8.dispatch(d, route_match, body))
            }
            J(d) => {
                J(self.9.dispatch(d, route_match, body))
            }
            K(d) => {
                K(self.10.dispatch(d, route_match, body))
            }
        }
    }
}
// ===== 12 =====

#[derive(Debug, Clone)]
pub enum Either12<A, B, C, D, E, F, G, H, I, J, K, L> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
    F(F),
    G(G),
    H(H),
    I(I),
    J(J),
    K(K),
    L(L),
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> Future for Either12<A, B, C, D, E, F, G, H, I, J, K, L>
where
    A: Future,
    B: Future<Error = A::Error>,
    C: Future<Error = A::Error>,
    D: Future<Error = A::Error>,
    E: Future<Error = A::Error>,
    F: Future<Error = A::Error>,
    G: Future<Error = A::Error>,
    H: Future<Error = A::Error>,
    I: Future<Error = A::Error>,
    J: Future<Error = A::Error>,
    K: Future<Error = A::Error>,
    L: Future<Error = A::Error>,
{
    type Item = Either12<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item, I::Item, J::Item, K::Item, L::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either12::*;

        match *self {
            A(ref mut f) => Ok(A(try_ready!(f.poll())).into()),
            B(ref mut f) => Ok(B(try_ready!(f.poll())).into()),
            C(ref mut f) => Ok(C(try_ready!(f.poll())).into()),
            D(ref mut f) => Ok(D(try_ready!(f.poll())).into()),
            E(ref mut f) => Ok(E(try_ready!(f.poll())).into()),
            F(ref mut f) => Ok(F(try_ready!(f.poll())).into()),
            G(ref mut f) => Ok(G(try_ready!(f.poll())).into()),
            H(ref mut f) => Ok(H(try_ready!(f.poll())).into()),
            I(ref mut f) => Ok(I(try_ready!(f.poll())).into()),
            J(ref mut f) => Ok(J(try_ready!(f.poll())).into()),
            K(ref mut f) => Ok(K(try_ready!(f.poll())).into()),
            L(ref mut f) => Ok(L(try_ready!(f.poll())).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> ResourceFuture for Either12<A, B, C, D, E, F, G, H, I, J, K, L>
where
    A: ResourceFuture,
    B: ResourceFuture,
    C: ResourceFuture,
    D: ResourceFuture,
    E: ResourceFuture,
    F: ResourceFuture,
    G: ResourceFuture,
    H: ResourceFuture,
    I: ResourceFuture,
    J: ResourceFuture,
    K: ResourceFuture,
    L: ResourceFuture,
{
    type Body = Either12<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body, I::Body, J::Body, K::Body, L::Body>;

    fn poll_response(&mut self, request: &http::Request<()>) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either12::*;

        let response = match *self {
            A(ref mut f) => try_ready!(f.poll_response(request)).map(A),
            B(ref mut f) => try_ready!(f.poll_response(request)).map(B),
            C(ref mut f) => try_ready!(f.poll_response(request)).map(C),
            D(ref mut f) => try_ready!(f.poll_response(request)).map(D),
            E(ref mut f) => try_ready!(f.poll_response(request)).map(E),
            F(ref mut f) => try_ready!(f.poll_response(request)).map(F),
            G(ref mut f) => try_ready!(f.poll_response(request)).map(G),
            H(ref mut f) => try_ready!(f.poll_response(request)).map(H),
            I(ref mut f) => try_ready!(f.poll_response(request)).map(I),
            J(ref mut f) => try_ready!(f.poll_response(request)).map(J),
            K(ref mut f) => try_ready!(f.poll_response(request)).map(K),
            L(ref mut f) => try_ready!(f.poll_response(request)).map(L),
        };
        Ok(response.into())
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> Either12<A, B, C, D, E, F, G, H, I, J, K, L>
where
    A: ExtractFuture,
    B: ExtractFuture,
    C: ExtractFuture,
    D: ExtractFuture,
    E: ExtractFuture,
    F: ExtractFuture,
    G: ExtractFuture,
    H: ExtractFuture,
    I: ExtractFuture,
    J: ExtractFuture,
    K: ExtractFuture,
    L: ExtractFuture,
{

    pub fn poll_ready(&mut self) -> Poll<(), extract::Error> {
        use self::Either12::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
            C(ref mut f) => f.poll(),
            D(ref mut f) => f.poll(),
            E(ref mut f) => f.poll(),
            F(ref mut f) => f.poll(),
            G(ref mut f) => f.poll(),
            H(ref mut f) => f.poll(),
            I(ref mut f) => f.poll(),
            J(ref mut f) => f.poll(),
            K(ref mut f) => f.poll(),
            L(ref mut f) => f.poll(),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> HttpFuture for Either12<A, B, C, D, E, F, G, H, I, J, K, L>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
    G: HttpFuture,
    H: HttpFuture,
    I: HttpFuture,
    J: HttpFuture,
    K: HttpFuture,
    L: HttpFuture,
{
    type Body = Either12<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body, I::Body, J::Body, K::Body, L::Body>;

    fn poll_http(&mut self) -> Poll<http::Response<Self::Body>, crate::Error> {
        use self::Either12::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll_http()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll_http()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll_http()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll_http()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll_http()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll_http()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll_http()).map(H).into()),
            I(ref mut f) => Ok(try_ready!(f.poll_http()).map(I).into()),
            J(ref mut f) => Ok(try_ready!(f.poll_http()).map(J).into()),
            K(ref mut f) => Ok(try_ready!(f.poll_http()).map(K).into()),
            L(ref mut f) => Ok(try_ready!(f.poll_http()).map(L).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> SealedFuture for Either12<A, B, C, D, E, F, G, H, I, J, K, L>
where
    A: HttpFuture,
    B: HttpFuture,
    C: HttpFuture,
    D: HttpFuture,
    E: HttpFuture,
    F: HttpFuture,
    G: HttpFuture,
    H: HttpFuture,
    I: HttpFuture,
    J: HttpFuture,
    K: HttpFuture,
    L: HttpFuture,
{
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> Stream for Either12<A, B, C, D, E, F, G, H, I, J, K, L>
where
    A: Stream,
    B: Stream<Error = A::Error>,
    C: Stream<Error = A::Error>,
    D: Stream<Error = A::Error>,
    E: Stream<Error = A::Error>,
    F: Stream<Error = A::Error>,
    G: Stream<Error = A::Error>,
    H: Stream<Error = A::Error>,
    I: Stream<Error = A::Error>,
    J: Stream<Error = A::Error>,
    K: Stream<Error = A::Error>,
    L: Stream<Error = A::Error>,
{
    type Item = Either12<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item, I::Item, J::Item, K::Item, L::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either12::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll()).map(H).into()),
            I(ref mut f) => Ok(try_ready!(f.poll()).map(I).into()),
            J(ref mut f) => Ok(try_ready!(f.poll()).map(J).into()),
            K(ref mut f) => Ok(try_ready!(f.poll()).map(K).into()),
            L(ref mut f) => Ok(try_ready!(f.poll()).map(L).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> BufStream for Either12<A, B, C, D, E, F, G, H, I, J, K, L>
where
    A: BufStream,
    B: BufStream<Error = A::Error>,
    C: BufStream<Error = A::Error>,
    D: BufStream<Error = A::Error>,
    E: BufStream<Error = A::Error>,
    F: BufStream<Error = A::Error>,
    G: BufStream<Error = A::Error>,
    H: BufStream<Error = A::Error>,
    I: BufStream<Error = A::Error>,
    J: BufStream<Error = A::Error>,
    K: BufStream<Error = A::Error>,
    L: BufStream<Error = A::Error>,
{
    type Item = Either12<A::Item, B::Item, C::Item, D::Item, E::Item, F::Item, G::Item, H::Item, I::Item, J::Item, K::Item, L::Item>;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::Either12::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
            D(ref mut f) => Ok(try_ready!(f.poll()).map(D).into()),
            E(ref mut f) => Ok(try_ready!(f.poll()).map(E).into()),
            F(ref mut f) => Ok(try_ready!(f.poll()).map(F).into()),
            G(ref mut f) => Ok(try_ready!(f.poll()).map(G).into()),
            H(ref mut f) => Ok(try_ready!(f.poll()).map(H).into()),
            I(ref mut f) => Ok(try_ready!(f.poll()).map(I).into()),
            J(ref mut f) => Ok(try_ready!(f.poll()).map(J).into()),
            K(ref mut f) => Ok(try_ready!(f.poll()).map(K).into()),
            L(ref mut f) => Ok(try_ready!(f.poll()).map(L).into()),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> Buf for Either12<A, B, C, D, E, F, G, H, I, J, K, L>
where
    A: Buf,
    B: Buf,
    C: Buf,
    D: Buf,
    E: Buf,
    F: Buf,
    G: Buf,
    H: Buf,
    I: Buf,
    J: Buf,
    K: Buf,
    L: Buf,
{
    fn remaining(&self) -> usize {
        use self::Either12::*;

        match *self {
            A(ref b) => b.remaining(),
            B(ref b) => b.remaining(),
            C(ref b) => b.remaining(),
            D(ref b) => b.remaining(),
            E(ref b) => b.remaining(),
            F(ref b) => b.remaining(),
            G(ref b) => b.remaining(),
            H(ref b) => b.remaining(),
            I(ref b) => b.remaining(),
            J(ref b) => b.remaining(),
            K(ref b) => b.remaining(),
            L(ref b) => b.remaining(),
        }
    }

    fn bytes(&self) -> &[u8] {
        use self::Either12::*;

        match *self {
            A(ref b) => b.bytes(),
            B(ref b) => b.bytes(),
            C(ref b) => b.bytes(),
            D(ref b) => b.bytes(),
            E(ref b) => b.bytes(),
            F(ref b) => b.bytes(),
            G(ref b) => b.bytes(),
            H(ref b) => b.bytes(),
            I(ref b) => b.bytes(),
            J(ref b) => b.bytes(),
            K(ref b) => b.bytes(),
            L(ref b) => b.bytes(),
        }
    }

    fn advance(&mut self, cnt: usize) {
        use self::Either12::*;

        match *self {
            A(ref mut b) => b.advance(cnt),
            B(ref mut b) => b.advance(cnt),
            C(ref mut b) => b.advance(cnt),
            D(ref mut b) => b.advance(cnt),
            E(ref mut b) => b.advance(cnt),
            F(ref mut b) => b.advance(cnt),
            G(ref mut b) => b.advance(cnt),
            H(ref mut b) => b.advance(cnt),
            I(ref mut b) => b.advance(cnt),
            J(ref mut b) => b.advance(cnt),
            K(ref mut b) => b.advance(cnt),
            L(ref mut b) => b.advance(cnt),
        }
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> Response for Either12<A, B, C, D, E, F, G, H, I, J, K, L>
where
    A: Response,
    B: Response,
    C: Response,
    D: Response,
    E: Response,
    F: Response,
    G: Response,
    H: Response,
    I: Response,
    J: Response,
    K: Response,
    L: Response,
{
    type Buf = Either12<A::Buf, B::Buf, C::Buf, D::Buf, E::Buf, F::Buf, G::Buf, H::Buf, I::Buf, J::Buf, K::Buf, L::Buf>;
    type Body = Either12<A::Body, B::Body, C::Body, D::Body, E::Body, F::Body, G::Body, H::Body, I::Body, J::Body, K::Body, L::Body>;

    fn into_http<S>(self, context: &Context<S>) -> Result<http::Response<Self::Body>, crate::Error>
    where S: Serializer
    {
        use self::Either12::*;

        match self {
            A(r) => Ok(r.into_http(context)?.map(Either12::A)),
            B(r) => Ok(r.into_http(context)?.map(Either12::B)),
            C(r) => Ok(r.into_http(context)?.map(Either12::C)),
            D(r) => Ok(r.into_http(context)?.map(Either12::D)),
            E(r) => Ok(r.into_http(context)?.map(Either12::E)),
            F(r) => Ok(r.into_http(context)?.map(Either12::F)),
            G(r) => Ok(r.into_http(context)?.map(Either12::G)),
            H(r) => Ok(r.into_http(context)?.map(Either12::H)),
            I(r) => Ok(r.into_http(context)?.map(Either12::I)),
            J(r) => Ok(r.into_http(context)?.map(Either12::J)),
            K(r) => Ok(r.into_http(context)?.map(Either12::K)),
            L(r) => Ok(r.into_http(context)?.map(Either12::L)),
        }
    }
}

impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11> Resource for (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11,)
where
    R0: Resource,
    R1: Resource<RequestBody = R0::RequestBody>,
    R2: Resource<RequestBody = R0::RequestBody>,
    R3: Resource<RequestBody = R0::RequestBody>,
    R4: Resource<RequestBody = R0::RequestBody>,
    R5: Resource<RequestBody = R0::RequestBody>,
    R6: Resource<RequestBody = R0::RequestBody>,
    R7: Resource<RequestBody = R0::RequestBody>,
    R8: Resource<RequestBody = R0::RequestBody>,
    R9: Resource<RequestBody = R0::RequestBody>,
    R10: Resource<RequestBody = R0::RequestBody>,
    R11: Resource<RequestBody = R0::RequestBody>,
{
    type Destination = Either12<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination, R5::Destination, R6::Destination, R7::Destination, R8::Destination, R9::Destination, R10::Destination, R11::Destination>;
    type RequestBody = R0::RequestBody;
    type Buf = Either12<R0::Buf, R1::Buf, R2::Buf, R3::Buf, R4::Buf, R5::Buf, R6::Buf, R7::Buf, R8::Buf, R9::Buf, R10::Buf, R11::Buf>;
    type Body = Either12<R0::Body, R1::Body, R2::Body, R3::Body, R4::Body, R5::Body, R6::Body, R7::Body, R8::Body, R9::Body, R10::Body, R11::Body>;
    type Future = Either12<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future, R5::Future, R6::Future, R7::Future, R8::Future, R9::Future, R10::Future, R11::Future>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: &RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either12::*;

        match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, body))
            }
            C(d) => {
                C(self.2.dispatch(d, route_match, body))
            }
            D(d) => {
                D(self.3.dispatch(d, route_match, body))
            }
            E(d) => {
                E(self.4.dispatch(d, route_match, body))
            }
            F(d) => {
                F(self.5.dispatch(d, route_match, body))
            }
            G(d) => {
                G(self.6.dispatch(d, route_match, body))
            }
            H(d) => {
                H(self.7.dispatch(d, route_match, body))
            }
            I(d) => {
                I(self.8.dispatch(d, route_match, body))
            }
            J(d) => {
                J(self.9.dispatch(d, route_match, body))
            }
            K(d) => {
                K(self.10.dispatch(d, route_match, body))
            }
            L(d) => {
                L(self.11.dispatch(d, route_match, body))
            }
        }
    }
}

impl<R0, U> Chain<U> for (R0,) {
    type Output = (R0, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, other)
    }
}

#[derive(Debug)]
pub struct Join1<T0> {
    futures: (T0,),
    pending: (bool,),
}

impl<T0> Join1<T0>
where
    T0: ExtractFuture,
{
    pub fn new(t0: T0) -> Self {
        Self {
            pending: (false, ),
            futures: (t0, ),
        }
    }

    pub fn into_inner(self) -> (T0,)
    {
        self.futures
    }
}

impl<T0> Future for Join1<T0>
where
    T0: ExtractFuture,
{
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
    let mut all_ready = true;

        if !self.pending.0 {
            self.pending.0 = self.futures.0.poll()?.is_ready();
            all_ready &= self.pending.0;
        }
        Ok(if all_ready { Async::Ready(()) } else { Async::NotReady })
    }
}
impl<S: Serializer, B: BufStream, T0> IntoResource<S, B> for (T0,)
where
    T0: IntoResource<S, B>,
{
    type Destination = Either1<T0::Destination>;
    type Resource = (T0::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        routes.insert_all(self.0.routes().map(Either1::A));
        routes.build()
    }

    fn into_resource(self, serializer: S) -> Self::Resource {
        (
            self.0.into_resource(serializer.clone()),
        )
    }
}
impl<R0, R1, U> Chain<U> for (R0, R1,) {
    type Output = (R0, R1, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, self.1, other)
    }
}

#[derive(Debug)]
pub struct Join2<T0, T1> {
    futures: (T0, T1,),
    pending: (bool, bool,),
}

impl<T0, T1> Join2<T0, T1>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
{
    pub fn new(t0: T0, t1: T1) -> Self {
        Self {
            pending: (false, false, ),
            futures: (t0, t1, ),
        }
    }

    pub fn into_inner(self) -> (T0, T1,)
    {
        self.futures
    }
}

impl<T0, T1> Future for Join2<T0, T1>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
{
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
    let mut all_ready = true;

        if !self.pending.0 {
            self.pending.0 = self.futures.0.poll()?.is_ready();
            all_ready &= self.pending.0;
        }
        if !self.pending.1 {
            self.pending.1 = self.futures.1.poll()?.is_ready();
            all_ready &= self.pending.1;
        }
        Ok(if all_ready { Async::Ready(()) } else { Async::NotReady })
    }
}
impl<S: Serializer, B: BufStream, T0, T1> IntoResource<S, B> for (T0, T1,)
where
    T0: IntoResource<S, B>,
    T1: IntoResource<S, B>,
{
    type Destination = Either2<T0::Destination, T1::Destination>;
    type Resource = (T0::Resource, T1::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        routes.insert_all(self.0.routes().map(Either2::A));
        routes.insert_all(self.1.routes().map(Either2::B));
        routes.build()
    }

    fn into_resource(self, serializer: S) -> Self::Resource {
        (
            self.0.into_resource(serializer.clone()),
            self.1.into_resource(serializer.clone()),
        )
    }
}
impl<R0, R1, R2, U> Chain<U> for (R0, R1, R2,) {
    type Output = (R0, R1, R2, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, self.1, self.2, other)
    }
}

#[derive(Debug)]
pub struct Join3<T0, T1, T2> {
    futures: (T0, T1, T2,),
    pending: (bool, bool, bool,),
}

impl<T0, T1, T2> Join3<T0, T1, T2>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
{
    pub fn new(t0: T0, t1: T1, t2: T2) -> Self {
        Self {
            pending: (false, false, false, ),
            futures: (t0, t1, t2, ),
        }
    }

    pub fn into_inner(self) -> (T0, T1, T2,)
    {
        self.futures
    }
}

impl<T0, T1, T2> Future for Join3<T0, T1, T2>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
{
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
    let mut all_ready = true;

        if !self.pending.0 {
            self.pending.0 = self.futures.0.poll()?.is_ready();
            all_ready &= self.pending.0;
        }
        if !self.pending.1 {
            self.pending.1 = self.futures.1.poll()?.is_ready();
            all_ready &= self.pending.1;
        }
        if !self.pending.2 {
            self.pending.2 = self.futures.2.poll()?.is_ready();
            all_ready &= self.pending.2;
        }
        Ok(if all_ready { Async::Ready(()) } else { Async::NotReady })
    }
}
impl<S: Serializer, B: BufStream, T0, T1, T2> IntoResource<S, B> for (T0, T1, T2,)
where
    T0: IntoResource<S, B>,
    T1: IntoResource<S, B>,
    T2: IntoResource<S, B>,
{
    type Destination = Either3<T0::Destination, T1::Destination, T2::Destination>;
    type Resource = (T0::Resource, T1::Resource, T2::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        routes.insert_all(self.0.routes().map(Either3::A));
        routes.insert_all(self.1.routes().map(Either3::B));
        routes.insert_all(self.2.routes().map(Either3::C));
        routes.build()
    }

    fn into_resource(self, serializer: S) -> Self::Resource {
        (
            self.0.into_resource(serializer.clone()),
            self.1.into_resource(serializer.clone()),
            self.2.into_resource(serializer.clone()),
        )
    }
}
impl<R0, R1, R2, R3, U> Chain<U> for (R0, R1, R2, R3,) {
    type Output = (R0, R1, R2, R3, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, self.1, self.2, self.3, other)
    }
}

#[derive(Debug)]
pub struct Join4<T0, T1, T2, T3> {
    futures: (T0, T1, T2, T3,),
    pending: (bool, bool, bool, bool,),
}

impl<T0, T1, T2, T3> Join4<T0, T1, T2, T3>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
{
    pub fn new(t0: T0, t1: T1, t2: T2, t3: T3) -> Self {
        Self {
            pending: (false, false, false, false, ),
            futures: (t0, t1, t2, t3, ),
        }
    }

    pub fn into_inner(self) -> (T0, T1, T2, T3,)
    {
        self.futures
    }
}

impl<T0, T1, T2, T3> Future for Join4<T0, T1, T2, T3>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
{
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
    let mut all_ready = true;

        if !self.pending.0 {
            self.pending.0 = self.futures.0.poll()?.is_ready();
            all_ready &= self.pending.0;
        }
        if !self.pending.1 {
            self.pending.1 = self.futures.1.poll()?.is_ready();
            all_ready &= self.pending.1;
        }
        if !self.pending.2 {
            self.pending.2 = self.futures.2.poll()?.is_ready();
            all_ready &= self.pending.2;
        }
        if !self.pending.3 {
            self.pending.3 = self.futures.3.poll()?.is_ready();
            all_ready &= self.pending.3;
        }
        Ok(if all_ready { Async::Ready(()) } else { Async::NotReady })
    }
}
impl<S: Serializer, B: BufStream, T0, T1, T2, T3> IntoResource<S, B> for (T0, T1, T2, T3,)
where
    T0: IntoResource<S, B>,
    T1: IntoResource<S, B>,
    T2: IntoResource<S, B>,
    T3: IntoResource<S, B>,
{
    type Destination = Either4<T0::Destination, T1::Destination, T2::Destination, T3::Destination>;
    type Resource = (T0::Resource, T1::Resource, T2::Resource, T3::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        routes.insert_all(self.0.routes().map(Either4::A));
        routes.insert_all(self.1.routes().map(Either4::B));
        routes.insert_all(self.2.routes().map(Either4::C));
        routes.insert_all(self.3.routes().map(Either4::D));
        routes.build()
    }

    fn into_resource(self, serializer: S) -> Self::Resource {
        (
            self.0.into_resource(serializer.clone()),
            self.1.into_resource(serializer.clone()),
            self.2.into_resource(serializer.clone()),
            self.3.into_resource(serializer.clone()),
        )
    }
}
impl<R0, R1, R2, R3, R4, U> Chain<U> for (R0, R1, R2, R3, R4,) {
    type Output = (R0, R1, R2, R3, R4, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, other)
    }
}

#[derive(Debug)]
pub struct Join5<T0, T1, T2, T3, T4> {
    futures: (T0, T1, T2, T3, T4,),
    pending: (bool, bool, bool, bool, bool,),
}

impl<T0, T1, T2, T3, T4> Join5<T0, T1, T2, T3, T4>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
{
    pub fn new(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4) -> Self {
        Self {
            pending: (false, false, false, false, false, ),
            futures: (t0, t1, t2, t3, t4, ),
        }
    }

    pub fn into_inner(self) -> (T0, T1, T2, T3, T4,)
    {
        self.futures
    }
}

impl<T0, T1, T2, T3, T4> Future for Join5<T0, T1, T2, T3, T4>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
{
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
    let mut all_ready = true;

        if !self.pending.0 {
            self.pending.0 = self.futures.0.poll()?.is_ready();
            all_ready &= self.pending.0;
        }
        if !self.pending.1 {
            self.pending.1 = self.futures.1.poll()?.is_ready();
            all_ready &= self.pending.1;
        }
        if !self.pending.2 {
            self.pending.2 = self.futures.2.poll()?.is_ready();
            all_ready &= self.pending.2;
        }
        if !self.pending.3 {
            self.pending.3 = self.futures.3.poll()?.is_ready();
            all_ready &= self.pending.3;
        }
        if !self.pending.4 {
            self.pending.4 = self.futures.4.poll()?.is_ready();
            all_ready &= self.pending.4;
        }
        Ok(if all_ready { Async::Ready(()) } else { Async::NotReady })
    }
}
impl<S: Serializer, B: BufStream, T0, T1, T2, T3, T4> IntoResource<S, B> for (T0, T1, T2, T3, T4,)
where
    T0: IntoResource<S, B>,
    T1: IntoResource<S, B>,
    T2: IntoResource<S, B>,
    T3: IntoResource<S, B>,
    T4: IntoResource<S, B>,
{
    type Destination = Either5<T0::Destination, T1::Destination, T2::Destination, T3::Destination, T4::Destination>;
    type Resource = (T0::Resource, T1::Resource, T2::Resource, T3::Resource, T4::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        routes.insert_all(self.0.routes().map(Either5::A));
        routes.insert_all(self.1.routes().map(Either5::B));
        routes.insert_all(self.2.routes().map(Either5::C));
        routes.insert_all(self.3.routes().map(Either5::D));
        routes.insert_all(self.4.routes().map(Either5::E));
        routes.build()
    }

    fn into_resource(self, serializer: S) -> Self::Resource {
        (
            self.0.into_resource(serializer.clone()),
            self.1.into_resource(serializer.clone()),
            self.2.into_resource(serializer.clone()),
            self.3.into_resource(serializer.clone()),
            self.4.into_resource(serializer.clone()),
        )
    }
}
impl<R0, R1, R2, R3, R4, R5, U> Chain<U> for (R0, R1, R2, R3, R4, R5,) {
    type Output = (R0, R1, R2, R3, R4, R5, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, self.5, other)
    }
}

#[derive(Debug)]
pub struct Join6<T0, T1, T2, T3, T4, T5> {
    futures: (T0, T1, T2, T3, T4, T5,),
    pending: (bool, bool, bool, bool, bool, bool,),
}

impl<T0, T1, T2, T3, T4, T5> Join6<T0, T1, T2, T3, T4, T5>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
{
    pub fn new(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5) -> Self {
        Self {
            pending: (false, false, false, false, false, false, ),
            futures: (t0, t1, t2, t3, t4, t5, ),
        }
    }

    pub fn into_inner(self) -> (T0, T1, T2, T3, T4, T5,)
    {
        self.futures
    }
}

impl<T0, T1, T2, T3, T4, T5> Future for Join6<T0, T1, T2, T3, T4, T5>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
{
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
    let mut all_ready = true;

        if !self.pending.0 {
            self.pending.0 = self.futures.0.poll()?.is_ready();
            all_ready &= self.pending.0;
        }
        if !self.pending.1 {
            self.pending.1 = self.futures.1.poll()?.is_ready();
            all_ready &= self.pending.1;
        }
        if !self.pending.2 {
            self.pending.2 = self.futures.2.poll()?.is_ready();
            all_ready &= self.pending.2;
        }
        if !self.pending.3 {
            self.pending.3 = self.futures.3.poll()?.is_ready();
            all_ready &= self.pending.3;
        }
        if !self.pending.4 {
            self.pending.4 = self.futures.4.poll()?.is_ready();
            all_ready &= self.pending.4;
        }
        if !self.pending.5 {
            self.pending.5 = self.futures.5.poll()?.is_ready();
            all_ready &= self.pending.5;
        }
        Ok(if all_ready { Async::Ready(()) } else { Async::NotReady })
    }
}
impl<S: Serializer, B: BufStream, T0, T1, T2, T3, T4, T5> IntoResource<S, B> for (T0, T1, T2, T3, T4, T5,)
where
    T0: IntoResource<S, B>,
    T1: IntoResource<S, B>,
    T2: IntoResource<S, B>,
    T3: IntoResource<S, B>,
    T4: IntoResource<S, B>,
    T5: IntoResource<S, B>,
{
    type Destination = Either6<T0::Destination, T1::Destination, T2::Destination, T3::Destination, T4::Destination, T5::Destination>;
    type Resource = (T0::Resource, T1::Resource, T2::Resource, T3::Resource, T4::Resource, T5::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        routes.insert_all(self.0.routes().map(Either6::A));
        routes.insert_all(self.1.routes().map(Either6::B));
        routes.insert_all(self.2.routes().map(Either6::C));
        routes.insert_all(self.3.routes().map(Either6::D));
        routes.insert_all(self.4.routes().map(Either6::E));
        routes.insert_all(self.5.routes().map(Either6::F));
        routes.build()
    }

    fn into_resource(self, serializer: S) -> Self::Resource {
        (
            self.0.into_resource(serializer.clone()),
            self.1.into_resource(serializer.clone()),
            self.2.into_resource(serializer.clone()),
            self.3.into_resource(serializer.clone()),
            self.4.into_resource(serializer.clone()),
            self.5.into_resource(serializer.clone()),
        )
    }
}
impl<R0, R1, R2, R3, R4, R5, R6, U> Chain<U> for (R0, R1, R2, R3, R4, R5, R6,) {
    type Output = (R0, R1, R2, R3, R4, R5, R6, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, other)
    }
}

#[derive(Debug)]
pub struct Join7<T0, T1, T2, T3, T4, T5, T6> {
    futures: (T0, T1, T2, T3, T4, T5, T6,),
    pending: (bool, bool, bool, bool, bool, bool, bool,),
}

impl<T0, T1, T2, T3, T4, T5, T6> Join7<T0, T1, T2, T3, T4, T5, T6>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
    T6: ExtractFuture,
{
    pub fn new(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6) -> Self {
        Self {
            pending: (false, false, false, false, false, false, false, ),
            futures: (t0, t1, t2, t3, t4, t5, t6, ),
        }
    }

    pub fn into_inner(self) -> (T0, T1, T2, T3, T4, T5, T6,)
    {
        self.futures
    }
}

impl<T0, T1, T2, T3, T4, T5, T6> Future for Join7<T0, T1, T2, T3, T4, T5, T6>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
    T6: ExtractFuture,
{
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
    let mut all_ready = true;

        if !self.pending.0 {
            self.pending.0 = self.futures.0.poll()?.is_ready();
            all_ready &= self.pending.0;
        }
        if !self.pending.1 {
            self.pending.1 = self.futures.1.poll()?.is_ready();
            all_ready &= self.pending.1;
        }
        if !self.pending.2 {
            self.pending.2 = self.futures.2.poll()?.is_ready();
            all_ready &= self.pending.2;
        }
        if !self.pending.3 {
            self.pending.3 = self.futures.3.poll()?.is_ready();
            all_ready &= self.pending.3;
        }
        if !self.pending.4 {
            self.pending.4 = self.futures.4.poll()?.is_ready();
            all_ready &= self.pending.4;
        }
        if !self.pending.5 {
            self.pending.5 = self.futures.5.poll()?.is_ready();
            all_ready &= self.pending.5;
        }
        if !self.pending.6 {
            self.pending.6 = self.futures.6.poll()?.is_ready();
            all_ready &= self.pending.6;
        }
        Ok(if all_ready { Async::Ready(()) } else { Async::NotReady })
    }
}
impl<S: Serializer, B: BufStream, T0, T1, T2, T3, T4, T5, T6> IntoResource<S, B> for (T0, T1, T2, T3, T4, T5, T6,)
where
    T0: IntoResource<S, B>,
    T1: IntoResource<S, B>,
    T2: IntoResource<S, B>,
    T3: IntoResource<S, B>,
    T4: IntoResource<S, B>,
    T5: IntoResource<S, B>,
    T6: IntoResource<S, B>,
{
    type Destination = Either7<T0::Destination, T1::Destination, T2::Destination, T3::Destination, T4::Destination, T5::Destination, T6::Destination>;
    type Resource = (T0::Resource, T1::Resource, T2::Resource, T3::Resource, T4::Resource, T5::Resource, T6::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        routes.insert_all(self.0.routes().map(Either7::A));
        routes.insert_all(self.1.routes().map(Either7::B));
        routes.insert_all(self.2.routes().map(Either7::C));
        routes.insert_all(self.3.routes().map(Either7::D));
        routes.insert_all(self.4.routes().map(Either7::E));
        routes.insert_all(self.5.routes().map(Either7::F));
        routes.insert_all(self.6.routes().map(Either7::G));
        routes.build()
    }

    fn into_resource(self, serializer: S) -> Self::Resource {
        (
            self.0.into_resource(serializer.clone()),
            self.1.into_resource(serializer.clone()),
            self.2.into_resource(serializer.clone()),
            self.3.into_resource(serializer.clone()),
            self.4.into_resource(serializer.clone()),
            self.5.into_resource(serializer.clone()),
            self.6.into_resource(serializer.clone()),
        )
    }
}
impl<R0, R1, R2, R3, R4, R5, R6, R7, U> Chain<U> for (R0, R1, R2, R3, R4, R5, R6, R7,) {
    type Output = (R0, R1, R2, R3, R4, R5, R6, R7, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, other)
    }
}

#[derive(Debug)]
pub struct Join8<T0, T1, T2, T3, T4, T5, T6, T7> {
    futures: (T0, T1, T2, T3, T4, T5, T6, T7,),
    pending: (bool, bool, bool, bool, bool, bool, bool, bool,),
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> Join8<T0, T1, T2, T3, T4, T5, T6, T7>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
    T6: ExtractFuture,
    T7: ExtractFuture,
{
    pub fn new(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7) -> Self {
        Self {
            pending: (false, false, false, false, false, false, false, false, ),
            futures: (t0, t1, t2, t3, t4, t5, t6, t7, ),
        }
    }

    pub fn into_inner(self) -> (T0, T1, T2, T3, T4, T5, T6, T7,)
    {
        self.futures
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7> Future for Join8<T0, T1, T2, T3, T4, T5, T6, T7>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
    T6: ExtractFuture,
    T7: ExtractFuture,
{
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
    let mut all_ready = true;

        if !self.pending.0 {
            self.pending.0 = self.futures.0.poll()?.is_ready();
            all_ready &= self.pending.0;
        }
        if !self.pending.1 {
            self.pending.1 = self.futures.1.poll()?.is_ready();
            all_ready &= self.pending.1;
        }
        if !self.pending.2 {
            self.pending.2 = self.futures.2.poll()?.is_ready();
            all_ready &= self.pending.2;
        }
        if !self.pending.3 {
            self.pending.3 = self.futures.3.poll()?.is_ready();
            all_ready &= self.pending.3;
        }
        if !self.pending.4 {
            self.pending.4 = self.futures.4.poll()?.is_ready();
            all_ready &= self.pending.4;
        }
        if !self.pending.5 {
            self.pending.5 = self.futures.5.poll()?.is_ready();
            all_ready &= self.pending.5;
        }
        if !self.pending.6 {
            self.pending.6 = self.futures.6.poll()?.is_ready();
            all_ready &= self.pending.6;
        }
        if !self.pending.7 {
            self.pending.7 = self.futures.7.poll()?.is_ready();
            all_ready &= self.pending.7;
        }
        Ok(if all_ready { Async::Ready(()) } else { Async::NotReady })
    }
}
impl<S: Serializer, B: BufStream, T0, T1, T2, T3, T4, T5, T6, T7> IntoResource<S, B> for (T0, T1, T2, T3, T4, T5, T6, T7,)
where
    T0: IntoResource<S, B>,
    T1: IntoResource<S, B>,
    T2: IntoResource<S, B>,
    T3: IntoResource<S, B>,
    T4: IntoResource<S, B>,
    T5: IntoResource<S, B>,
    T6: IntoResource<S, B>,
    T7: IntoResource<S, B>,
{
    type Destination = Either8<T0::Destination, T1::Destination, T2::Destination, T3::Destination, T4::Destination, T5::Destination, T6::Destination, T7::Destination>;
    type Resource = (T0::Resource, T1::Resource, T2::Resource, T3::Resource, T4::Resource, T5::Resource, T6::Resource, T7::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        routes.insert_all(self.0.routes().map(Either8::A));
        routes.insert_all(self.1.routes().map(Either8::B));
        routes.insert_all(self.2.routes().map(Either8::C));
        routes.insert_all(self.3.routes().map(Either8::D));
        routes.insert_all(self.4.routes().map(Either8::E));
        routes.insert_all(self.5.routes().map(Either8::F));
        routes.insert_all(self.6.routes().map(Either8::G));
        routes.insert_all(self.7.routes().map(Either8::H));
        routes.build()
    }

    fn into_resource(self, serializer: S) -> Self::Resource {
        (
            self.0.into_resource(serializer.clone()),
            self.1.into_resource(serializer.clone()),
            self.2.into_resource(serializer.clone()),
            self.3.into_resource(serializer.clone()),
            self.4.into_resource(serializer.clone()),
            self.5.into_resource(serializer.clone()),
            self.6.into_resource(serializer.clone()),
            self.7.into_resource(serializer.clone()),
        )
    }
}
impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, U> Chain<U> for (R0, R1, R2, R3, R4, R5, R6, R7, R8,) {
    type Output = (R0, R1, R2, R3, R4, R5, R6, R7, R8, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, other)
    }
}

#[derive(Debug)]
pub struct Join9<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    futures: (T0, T1, T2, T3, T4, T5, T6, T7, T8,),
    pending: (bool, bool, bool, bool, bool, bool, bool, bool, bool,),
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Join9<T0, T1, T2, T3, T4, T5, T6, T7, T8>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
    T6: ExtractFuture,
    T7: ExtractFuture,
    T8: ExtractFuture,
{
    pub fn new(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8) -> Self {
        Self {
            pending: (false, false, false, false, false, false, false, false, false, ),
            futures: (t0, t1, t2, t3, t4, t5, t6, t7, t8, ),
        }
    }

    pub fn into_inner(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8,)
    {
        self.futures
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Future for Join9<T0, T1, T2, T3, T4, T5, T6, T7, T8>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
    T6: ExtractFuture,
    T7: ExtractFuture,
    T8: ExtractFuture,
{
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
    let mut all_ready = true;

        if !self.pending.0 {
            self.pending.0 = self.futures.0.poll()?.is_ready();
            all_ready &= self.pending.0;
        }
        if !self.pending.1 {
            self.pending.1 = self.futures.1.poll()?.is_ready();
            all_ready &= self.pending.1;
        }
        if !self.pending.2 {
            self.pending.2 = self.futures.2.poll()?.is_ready();
            all_ready &= self.pending.2;
        }
        if !self.pending.3 {
            self.pending.3 = self.futures.3.poll()?.is_ready();
            all_ready &= self.pending.3;
        }
        if !self.pending.4 {
            self.pending.4 = self.futures.4.poll()?.is_ready();
            all_ready &= self.pending.4;
        }
        if !self.pending.5 {
            self.pending.5 = self.futures.5.poll()?.is_ready();
            all_ready &= self.pending.5;
        }
        if !self.pending.6 {
            self.pending.6 = self.futures.6.poll()?.is_ready();
            all_ready &= self.pending.6;
        }
        if !self.pending.7 {
            self.pending.7 = self.futures.7.poll()?.is_ready();
            all_ready &= self.pending.7;
        }
        if !self.pending.8 {
            self.pending.8 = self.futures.8.poll()?.is_ready();
            all_ready &= self.pending.8;
        }
        Ok(if all_ready { Async::Ready(()) } else { Async::NotReady })
    }
}
impl<S: Serializer, B: BufStream, T0, T1, T2, T3, T4, T5, T6, T7, T8> IntoResource<S, B> for (T0, T1, T2, T3, T4, T5, T6, T7, T8,)
where
    T0: IntoResource<S, B>,
    T1: IntoResource<S, B>,
    T2: IntoResource<S, B>,
    T3: IntoResource<S, B>,
    T4: IntoResource<S, B>,
    T5: IntoResource<S, B>,
    T6: IntoResource<S, B>,
    T7: IntoResource<S, B>,
    T8: IntoResource<S, B>,
{
    type Destination = Either9<T0::Destination, T1::Destination, T2::Destination, T3::Destination, T4::Destination, T5::Destination, T6::Destination, T7::Destination, T8::Destination>;
    type Resource = (T0::Resource, T1::Resource, T2::Resource, T3::Resource, T4::Resource, T5::Resource, T6::Resource, T7::Resource, T8::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        routes.insert_all(self.0.routes().map(Either9::A));
        routes.insert_all(self.1.routes().map(Either9::B));
        routes.insert_all(self.2.routes().map(Either9::C));
        routes.insert_all(self.3.routes().map(Either9::D));
        routes.insert_all(self.4.routes().map(Either9::E));
        routes.insert_all(self.5.routes().map(Either9::F));
        routes.insert_all(self.6.routes().map(Either9::G));
        routes.insert_all(self.7.routes().map(Either9::H));
        routes.insert_all(self.8.routes().map(Either9::I));
        routes.build()
    }

    fn into_resource(self, serializer: S) -> Self::Resource {
        (
            self.0.into_resource(serializer.clone()),
            self.1.into_resource(serializer.clone()),
            self.2.into_resource(serializer.clone()),
            self.3.into_resource(serializer.clone()),
            self.4.into_resource(serializer.clone()),
            self.5.into_resource(serializer.clone()),
            self.6.into_resource(serializer.clone()),
            self.7.into_resource(serializer.clone()),
            self.8.into_resource(serializer.clone()),
        )
    }
}
impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, U> Chain<U> for (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9,) {
    type Output = (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, other)
    }
}

#[derive(Debug)]
pub struct Join10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    futures: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,),
    pending: (bool, bool, bool, bool, bool, bool, bool, bool, bool, bool,),
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Join10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
    T6: ExtractFuture,
    T7: ExtractFuture,
    T8: ExtractFuture,
    T9: ExtractFuture,
{
    pub fn new(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9) -> Self {
        Self {
            pending: (false, false, false, false, false, false, false, false, false, false, ),
            futures: (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, ),
        }
    }

    pub fn into_inner(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,)
    {
        self.futures
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Future for Join10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
    T6: ExtractFuture,
    T7: ExtractFuture,
    T8: ExtractFuture,
    T9: ExtractFuture,
{
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
    let mut all_ready = true;

        if !self.pending.0 {
            self.pending.0 = self.futures.0.poll()?.is_ready();
            all_ready &= self.pending.0;
        }
        if !self.pending.1 {
            self.pending.1 = self.futures.1.poll()?.is_ready();
            all_ready &= self.pending.1;
        }
        if !self.pending.2 {
            self.pending.2 = self.futures.2.poll()?.is_ready();
            all_ready &= self.pending.2;
        }
        if !self.pending.3 {
            self.pending.3 = self.futures.3.poll()?.is_ready();
            all_ready &= self.pending.3;
        }
        if !self.pending.4 {
            self.pending.4 = self.futures.4.poll()?.is_ready();
            all_ready &= self.pending.4;
        }
        if !self.pending.5 {
            self.pending.5 = self.futures.5.poll()?.is_ready();
            all_ready &= self.pending.5;
        }
        if !self.pending.6 {
            self.pending.6 = self.futures.6.poll()?.is_ready();
            all_ready &= self.pending.6;
        }
        if !self.pending.7 {
            self.pending.7 = self.futures.7.poll()?.is_ready();
            all_ready &= self.pending.7;
        }
        if !self.pending.8 {
            self.pending.8 = self.futures.8.poll()?.is_ready();
            all_ready &= self.pending.8;
        }
        if !self.pending.9 {
            self.pending.9 = self.futures.9.poll()?.is_ready();
            all_ready &= self.pending.9;
        }
        Ok(if all_ready { Async::Ready(()) } else { Async::NotReady })
    }
}
impl<S: Serializer, B: BufStream, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> IntoResource<S, B> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,)
where
    T0: IntoResource<S, B>,
    T1: IntoResource<S, B>,
    T2: IntoResource<S, B>,
    T3: IntoResource<S, B>,
    T4: IntoResource<S, B>,
    T5: IntoResource<S, B>,
    T6: IntoResource<S, B>,
    T7: IntoResource<S, B>,
    T8: IntoResource<S, B>,
    T9: IntoResource<S, B>,
{
    type Destination = Either10<T0::Destination, T1::Destination, T2::Destination, T3::Destination, T4::Destination, T5::Destination, T6::Destination, T7::Destination, T8::Destination, T9::Destination>;
    type Resource = (T0::Resource, T1::Resource, T2::Resource, T3::Resource, T4::Resource, T5::Resource, T6::Resource, T7::Resource, T8::Resource, T9::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        routes.insert_all(self.0.routes().map(Either10::A));
        routes.insert_all(self.1.routes().map(Either10::B));
        routes.insert_all(self.2.routes().map(Either10::C));
        routes.insert_all(self.3.routes().map(Either10::D));
        routes.insert_all(self.4.routes().map(Either10::E));
        routes.insert_all(self.5.routes().map(Either10::F));
        routes.insert_all(self.6.routes().map(Either10::G));
        routes.insert_all(self.7.routes().map(Either10::H));
        routes.insert_all(self.8.routes().map(Either10::I));
        routes.insert_all(self.9.routes().map(Either10::J));
        routes.build()
    }

    fn into_resource(self, serializer: S) -> Self::Resource {
        (
            self.0.into_resource(serializer.clone()),
            self.1.into_resource(serializer.clone()),
            self.2.into_resource(serializer.clone()),
            self.3.into_resource(serializer.clone()),
            self.4.into_resource(serializer.clone()),
            self.5.into_resource(serializer.clone()),
            self.6.into_resource(serializer.clone()),
            self.7.into_resource(serializer.clone()),
            self.8.into_resource(serializer.clone()),
            self.9.into_resource(serializer.clone()),
        )
    }
}
impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, U> Chain<U> for (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10,) {
    type Output = (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, other)
    }
}

#[derive(Debug)]
pub struct Join11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    futures: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,),
    pending: (bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool,),
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Join11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
    T6: ExtractFuture,
    T7: ExtractFuture,
    T8: ExtractFuture,
    T9: ExtractFuture,
    T10: ExtractFuture,
{
    pub fn new(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10) -> Self {
        Self {
            pending: (false, false, false, false, false, false, false, false, false, false, false, ),
            futures: (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, ),
        }
    }

    pub fn into_inner(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,)
    {
        self.futures
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Future for Join11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
    T6: ExtractFuture,
    T7: ExtractFuture,
    T8: ExtractFuture,
    T9: ExtractFuture,
    T10: ExtractFuture,
{
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
    let mut all_ready = true;

        if !self.pending.0 {
            self.pending.0 = self.futures.0.poll()?.is_ready();
            all_ready &= self.pending.0;
        }
        if !self.pending.1 {
            self.pending.1 = self.futures.1.poll()?.is_ready();
            all_ready &= self.pending.1;
        }
        if !self.pending.2 {
            self.pending.2 = self.futures.2.poll()?.is_ready();
            all_ready &= self.pending.2;
        }
        if !self.pending.3 {
            self.pending.3 = self.futures.3.poll()?.is_ready();
            all_ready &= self.pending.3;
        }
        if !self.pending.4 {
            self.pending.4 = self.futures.4.poll()?.is_ready();
            all_ready &= self.pending.4;
        }
        if !self.pending.5 {
            self.pending.5 = self.futures.5.poll()?.is_ready();
            all_ready &= self.pending.5;
        }
        if !self.pending.6 {
            self.pending.6 = self.futures.6.poll()?.is_ready();
            all_ready &= self.pending.6;
        }
        if !self.pending.7 {
            self.pending.7 = self.futures.7.poll()?.is_ready();
            all_ready &= self.pending.7;
        }
        if !self.pending.8 {
            self.pending.8 = self.futures.8.poll()?.is_ready();
            all_ready &= self.pending.8;
        }
        if !self.pending.9 {
            self.pending.9 = self.futures.9.poll()?.is_ready();
            all_ready &= self.pending.9;
        }
        if !self.pending.10 {
            self.pending.10 = self.futures.10.poll()?.is_ready();
            all_ready &= self.pending.10;
        }
        Ok(if all_ready { Async::Ready(()) } else { Async::NotReady })
    }
}
impl<S: Serializer, B: BufStream, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> IntoResource<S, B> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,)
where
    T0: IntoResource<S, B>,
    T1: IntoResource<S, B>,
    T2: IntoResource<S, B>,
    T3: IntoResource<S, B>,
    T4: IntoResource<S, B>,
    T5: IntoResource<S, B>,
    T6: IntoResource<S, B>,
    T7: IntoResource<S, B>,
    T8: IntoResource<S, B>,
    T9: IntoResource<S, B>,
    T10: IntoResource<S, B>,
{
    type Destination = Either11<T0::Destination, T1::Destination, T2::Destination, T3::Destination, T4::Destination, T5::Destination, T6::Destination, T7::Destination, T8::Destination, T9::Destination, T10::Destination>;
    type Resource = (T0::Resource, T1::Resource, T2::Resource, T3::Resource, T4::Resource, T5::Resource, T6::Resource, T7::Resource, T8::Resource, T9::Resource, T10::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        routes.insert_all(self.0.routes().map(Either11::A));
        routes.insert_all(self.1.routes().map(Either11::B));
        routes.insert_all(self.2.routes().map(Either11::C));
        routes.insert_all(self.3.routes().map(Either11::D));
        routes.insert_all(self.4.routes().map(Either11::E));
        routes.insert_all(self.5.routes().map(Either11::F));
        routes.insert_all(self.6.routes().map(Either11::G));
        routes.insert_all(self.7.routes().map(Either11::H));
        routes.insert_all(self.8.routes().map(Either11::I));
        routes.insert_all(self.9.routes().map(Either11::J));
        routes.insert_all(self.10.routes().map(Either11::K));
        routes.build()
    }

    fn into_resource(self, serializer: S) -> Self::Resource {
        (
            self.0.into_resource(serializer.clone()),
            self.1.into_resource(serializer.clone()),
            self.2.into_resource(serializer.clone()),
            self.3.into_resource(serializer.clone()),
            self.4.into_resource(serializer.clone()),
            self.5.into_resource(serializer.clone()),
            self.6.into_resource(serializer.clone()),
            self.7.into_resource(serializer.clone()),
            self.8.into_resource(serializer.clone()),
            self.9.into_resource(serializer.clone()),
            self.10.into_resource(serializer.clone()),
        )
    }
}
impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, U> Chain<U> for (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11,) {
    type Output = (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, other)
    }
}

#[derive(Debug)]
pub struct Join12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    futures: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,),
    pending: (bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool, bool,),
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Join12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
    T6: ExtractFuture,
    T7: ExtractFuture,
    T8: ExtractFuture,
    T9: ExtractFuture,
    T10: ExtractFuture,
    T11: ExtractFuture,
{
    pub fn new(t0: T0, t1: T1, t2: T2, t3: T3, t4: T4, t5: T5, t6: T6, t7: T7, t8: T8, t9: T9, t10: T10, t11: T11) -> Self {
        Self {
            pending: (false, false, false, false, false, false, false, false, false, false, false, false, ),
            futures: (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, ),
        }
    }

    pub fn into_inner(self) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,)
    {
        self.futures
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Future for Join12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
where
    T0: ExtractFuture,
    T1: ExtractFuture,
    T2: ExtractFuture,
    T3: ExtractFuture,
    T4: ExtractFuture,
    T5: ExtractFuture,
    T6: ExtractFuture,
    T7: ExtractFuture,
    T8: ExtractFuture,
    T9: ExtractFuture,
    T10: ExtractFuture,
    T11: ExtractFuture,
{
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
    let mut all_ready = true;

        if !self.pending.0 {
            self.pending.0 = self.futures.0.poll()?.is_ready();
            all_ready &= self.pending.0;
        }
        if !self.pending.1 {
            self.pending.1 = self.futures.1.poll()?.is_ready();
            all_ready &= self.pending.1;
        }
        if !self.pending.2 {
            self.pending.2 = self.futures.2.poll()?.is_ready();
            all_ready &= self.pending.2;
        }
        if !self.pending.3 {
            self.pending.3 = self.futures.3.poll()?.is_ready();
            all_ready &= self.pending.3;
        }
        if !self.pending.4 {
            self.pending.4 = self.futures.4.poll()?.is_ready();
            all_ready &= self.pending.4;
        }
        if !self.pending.5 {
            self.pending.5 = self.futures.5.poll()?.is_ready();
            all_ready &= self.pending.5;
        }
        if !self.pending.6 {
            self.pending.6 = self.futures.6.poll()?.is_ready();
            all_ready &= self.pending.6;
        }
        if !self.pending.7 {
            self.pending.7 = self.futures.7.poll()?.is_ready();
            all_ready &= self.pending.7;
        }
        if !self.pending.8 {
            self.pending.8 = self.futures.8.poll()?.is_ready();
            all_ready &= self.pending.8;
        }
        if !self.pending.9 {
            self.pending.9 = self.futures.9.poll()?.is_ready();
            all_ready &= self.pending.9;
        }
        if !self.pending.10 {
            self.pending.10 = self.futures.10.poll()?.is_ready();
            all_ready &= self.pending.10;
        }
        if !self.pending.11 {
            self.pending.11 = self.futures.11.poll()?.is_ready();
            all_ready &= self.pending.11;
        }
        Ok(if all_ready { Async::Ready(()) } else { Async::NotReady })
    }
}
impl<S: Serializer, B: BufStream, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> IntoResource<S, B> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,)
where
    T0: IntoResource<S, B>,
    T1: IntoResource<S, B>,
    T2: IntoResource<S, B>,
    T3: IntoResource<S, B>,
    T4: IntoResource<S, B>,
    T5: IntoResource<S, B>,
    T6: IntoResource<S, B>,
    T7: IntoResource<S, B>,
    T8: IntoResource<S, B>,
    T9: IntoResource<S, B>,
    T10: IntoResource<S, B>,
    T11: IntoResource<S, B>,
{
    type Destination = Either12<T0::Destination, T1::Destination, T2::Destination, T3::Destination, T4::Destination, T5::Destination, T6::Destination, T7::Destination, T8::Destination, T9::Destination, T10::Destination, T11::Destination>;
    type Resource = (T0::Resource, T1::Resource, T2::Resource, T3::Resource, T4::Resource, T5::Resource, T6::Resource, T7::Resource, T8::Resource, T9::Resource, T10::Resource, T11::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        routes.insert_all(self.0.routes().map(Either12::A));
        routes.insert_all(self.1.routes().map(Either12::B));
        routes.insert_all(self.2.routes().map(Either12::C));
        routes.insert_all(self.3.routes().map(Either12::D));
        routes.insert_all(self.4.routes().map(Either12::E));
        routes.insert_all(self.5.routes().map(Either12::F));
        routes.insert_all(self.6.routes().map(Either12::G));
        routes.insert_all(self.7.routes().map(Either12::H));
        routes.insert_all(self.8.routes().map(Either12::I));
        routes.insert_all(self.9.routes().map(Either12::J));
        routes.insert_all(self.10.routes().map(Either12::K));
        routes.insert_all(self.11.routes().map(Either12::L));
        routes.build()
    }

    fn into_resource(self, serializer: S) -> Self::Resource {
        (
            self.0.into_resource(serializer.clone()),
            self.1.into_resource(serializer.clone()),
            self.2.into_resource(serializer.clone()),
            self.3.into_resource(serializer.clone()),
            self.4.into_resource(serializer.clone()),
            self.5.into_resource(serializer.clone()),
            self.6.into_resource(serializer.clone()),
            self.7.into_resource(serializer.clone()),
            self.8.into_resource(serializer.clone()),
            self.9.into_resource(serializer.clone()),
            self.10.into_resource(serializer.clone()),
            self.11.into_resource(serializer.clone()),
        )
    }
}
