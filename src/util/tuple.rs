//! Implementations of `Resource` for tuple types.

// NOTE: This file should not be updated directly. Instead, update
// `util/gen-tuple.rs` and regenerate this file.

use extract::{self, ExtractFuture};
use response::{Context, IntoResponse, MapErr, Serializer};
use routing::{self, RouteSet, RouteMatch};
use service::{Resource, IntoResource, HttpResponseFuture};
use util::{BufStream, Chain};

use bytes::Buf;
use futures::{Future, Stream, Async, Poll};
use futures::future::FutureResult;
use http;

// ===== Utility traits =====

pub struct LiftHttpResponse<T> {
    inner: T,
}

impl<T: HttpResponseFuture> Future for LiftHttpResponse<T> {
    type Item = http::Response<T::Item>;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Self::Item, ::Error> {
        self.inner.poll_http_response()
    }
}

// ===== 0 =====

impl Resource for () {
    type Destination = ();
    type Buf = <Self::Body as BufStream>::Item;
    type Body = MapErr<String>;
    type Future = FutureResult<http::Response<Self::Body>, ::Error>;

    fn dispatch<In: BufStream>(&mut self, _: (), _: RouteMatch, _: In) -> Self::Future {
        unreachable!();
    }
}

impl<S: Serializer> IntoResource<S> for () {
    type Destination = ();
    type Resource = ();

    fn routes(&self) -> RouteSet<()> {
        RouteSet::new()
    }

    fn into_resource(self, _: S) -> Self::Resource {
        ()
    }
}

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

#[derive(Clone)]
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

impl<A> HttpResponseFuture for Either1<A>
where
    A: HttpResponseFuture,
{
    type Item = Either1<A::Item>;

    fn poll_http_response(&mut self) -> Poll<http::Response<Self::Item>, ::Error> {
        use self::Either1::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http_response()).map(A).into()),
        }
    }
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

impl<A> IntoResponse for Either1<A>
where
    A: IntoResponse,
{
    type Buf = Either1<A::Buf>;
    type Body = Either1<A::Body>;

    fn into_response<S>(self, context: &Context<S>) ->  http::Response<Self::Body>
    where S: Serializer
    {
        use self::Either1::*;

        match self {
            A(r) => r.into_response(context).map(Either1::A),
        }
    }
}

impl<R0> Resource for (R0,)
where
    R0: Resource,
{
    type Destination = Either1<R0::Destination>;
    type Buf = Either1<R0::Buf>;
    type Body = Either1<R0::Body>;
    type Future = LiftHttpResponse<Either1<R0::Future>>;

    fn dispatch<In: BufStream>(&mut self,
                               destination: Self::Destination,
                               route_match: RouteMatch,
                               body: In)
        -> Self::Future
    {
        use self::Either1::*;

        let inner = match destination {
            A(d) => {
                A(self.0.dispatch(d, route_match, body))
            }
        };

        LiftHttpResponse { inner }
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

impl<A, B> HttpResponseFuture for Either2<A, B>
where
    A: HttpResponseFuture,
    B: HttpResponseFuture,
{
    type Item = Either2<A::Item, B::Item>;

    fn poll_http_response(&mut self) -> Poll<http::Response<Self::Item>, ::Error> {
        use self::Either2::*;

        match *self {
            A(ref mut f) => Ok(try_ready!(f.poll_http_response()).map(A).into()),
            B(ref mut f) => Ok(try_ready!(f.poll_http_response()).map(B).into()),
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

impl<R0, R1> Resource for (R0, R1,)
where
    R0: Resource,
    R1: Resource,
{
    type Destination = Either2<R0::Destination, R1::Destination>;
    type Buf = Either2<R0::Buf, R1::Buf>;
    type Body = Either2<R0::Body, R1::Body>;
    type Future = LiftHttpResponse<Either2<R0::Future, R1::Future>>;

    fn dispatch<In: BufStream>(&mut self,
                               destination: Self::Destination,
                               route_match: RouteMatch,
                               body: In)
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

        LiftHttpResponse { inner }
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
impl<S: Serializer, T0> IntoResource<S> for (T0,)
where
    T0: IntoResource<S>,
{
    type Destination = Either1<T0::Destination>;
    type Resource = (T0::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        for route in self.0.routes() {
            routes.push(route.map(Either1::A));
        }

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
impl<S: Serializer, T0, T1> IntoResource<S> for (T0, T1,)
where
    T0: IntoResource<S>,
    T1: IntoResource<S>,
{
    type Destination = Either2<T0::Destination, T1::Destination>;
    type Resource = (T0::Resource, T1::Resource,);

    fn routes(&self) -> RouteSet<Self::Destination>
    {
        let mut routes = routing::Builder::new();

        for route in self.0.routes() {
            routes.push(route.map(Either2::A));
        }

        for route in self.1.routes() {
            routes.push(route.map(Either2::B));
        }

        routes.build()
    }

    fn into_resource(self, serializer: S) -> Self::Resource {
        (
            self.0.into_resource(serializer.clone()),
            self.1.into_resource(serializer.clone()),
        )
    }
}
