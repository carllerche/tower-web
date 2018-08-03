//! Implementations of `Resource` for tuple types.

// NOTE: This file should not be updated directly. Instead, update
// `util/gen-tuple.rs` and regenerate this file.

use extract::{self, ExtractFuture};
use response::{Context, Response, Serializer};
use routing::{self, Resource, IntoResource, RouteSet, RouteMatch};
use util::{BufStream, Chain};
use util::http::{HttpFuture, LiftFuture, SealedFuture};

use bytes::Buf;
use futures::{Future, Stream, Async, Poll};
use http;

// ===== Utility traits =====



// ===== 0 =====

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

    fn poll(&mut self) -> Poll<http::Response<Self::Body>, ::Error> {
        use self::Either1::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
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

    fn into_http<S>(self, context: &Context<S>) ->  http::Response<Self::Body>
    where S: Serializer
    {
        use self::Either1::*;

        match self {
            A(r) => r.into_http(context).map(Either1::A),
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
    type Future = LiftFuture<Either1<R0::Future>>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either1::*;

        let inner = match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
        };

        inner.lift()
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

    fn poll(&mut self) -> Poll<http::Response<Self::Body>, ::Error> {
        use self::Either2::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
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

    fn into_http<S>(self, context: &Context<S>) ->  http::Response<Self::Body>
    where S: Serializer
    {
        use self::Either2::*;

        match self {
            A(r) => r.into_http(context).map(Either2::A),
            B(r) => r.into_http(context).map(Either2::B),
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
    type Future = LiftFuture<Either2<R0::Future, R1::Future>>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either2::*;

        let inner = match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, body))
            }
        };

        inner.lift()
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

    fn poll(&mut self) -> Poll<http::Response<Self::Body>, ::Error> {
        use self::Either3::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll()).map(B).into()),
            C(ref mut f) => Ok(try_ready!(f.poll()).map(C).into()),
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

    fn into_http<S>(self, context: &Context<S>) ->  http::Response<Self::Body>
    where S: Serializer
    {
        use self::Either3::*;

        match self {
            A(r) => r.into_http(context).map(Either3::A),
            B(r) => r.into_http(context).map(Either3::B),
            C(r) => r.into_http(context).map(Either3::C),
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
    type Future = LiftFuture<Either3<R0::Future, R1::Future, R2::Future>>;

    fn dispatch(&mut self,
                destination: Self::Destination,
                route_match: RouteMatch,
                body: Self::RequestBody)
        -> Self::Future
    {
        use self::Either3::*;

        let inner = match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
            B(d) => {
                B(self.1.dispatch(d, route_match, body))
            }
            C(d) => {
                C(self.2.dispatch(d, route_match, body))
            }
        };

        inner.lift()
    }
}

impl<R0, U> Chain<U> for (R0,) {
    type Output = (R0, U);

    fn chain(self, other: U) -> Self::Output {
        (self.0, other)
    }
}

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
