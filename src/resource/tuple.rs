//! Implementations of `Resource` for tuple types.

use super::{Chain, Resource};
use routing::{RouteSet, Match};

use bytes::Bytes;
use futures::{Future, Poll};
use futures::future::FutureResult;
use futures::stream::Once;
use http;

// ===== 0 =====

impl Resource for () {
    type Destination = ();
    type Body = Once<Bytes, ::Error>;
    type Future = FutureResult<http::Response<Self::Body>, ::Error>;

    fn routes(&self) -> RouteSet<()> {
        RouteSet::new()
    }

    fn dispatch(&mut self, _: Match<()>, _: http::Request<()>) -> Self::Future {
        unreachable!();
    }
}

impl<U> Chain<U> for () {
    type Resource = U;

    fn chain(self, other: U) -> Self::Resource {
        other
    }
}
// ===== 2 =====

#[derive(Clone)]
pub enum Either2<A = (), B = ()> {
    A(A),
    B(B),
}

impl<A, B> Future for Either2<A, B>
where
    A: Future,
    B: Future<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either2::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
        }
    }
}

impl<R0, R1> Resource for (R0, R1)
where
    R0: Resource,
    R1: Resource<Body = R0::Body>,
{
    type Destination = Either2<R0::Destination, R1::Destination>;
    type Body = R0::Body;
    type Future = Either2<R0::Future, R1::Future>;

    fn routes(&self) -> RouteSet<Self::Destination> {
        let mut routes = RouteSet::new();

        for route in self.0.routes() {
            routes.push(route.map(Either2::A));
        }

        for route in self.1.routes() {
            routes.push(route.map(Either2::B));
        }

        routes
    }

    fn dispatch(&mut self,
                match_: Match<Self::Destination>,
                request: http::Request<()>)
        -> Self::Future
    {
        use self::Either2::*;

        let (destination, params) = match_.into_parts();

        match destination {
            A(d) => {
                let match_ = Match::new(d, params);
                A(self.0.dispatch(match_, request))
            }
            B(d) => {
                let match_ = Match::new(d, params);
                B(self.1.dispatch(match_, request))
            }
        }
    }
}

impl<R0, R1, U> Chain<U> for (R0, R1) {
    type Resource = (R0, R1, U);

    fn chain(self, other: U) -> Self::Resource {
        (self.0, self.1, other)
    }
}
// ===== 3 =====

#[derive(Clone)]
pub enum Either3<A = (), B = (), C = ()> {
    A(A),
    B(B),
    C(C),
}

impl<A, B, C> Future for Either3<A, B, C>
where
    A: Future,
    B: Future<Item = A::Item, Error = A::Error>,
    C: Future<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either3::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
            C(ref mut f) => f.poll(),
        }
    }
}

impl<R0, R1, R2> Resource for (R0, R1, R2)
where
    R0: Resource,
    R1: Resource<Body = R0::Body>,
    R2: Resource<Body = R0::Body>,
{
    type Destination = Either3<R0::Destination, R1::Destination, R2::Destination>;
    type Body = R0::Body;
    type Future = Either3<R0::Future, R1::Future, R2::Future>;

    fn routes(&self) -> RouteSet<Self::Destination> {
        let mut routes = RouteSet::new();

        for route in self.0.routes() {
            routes.push(route.map(Either3::A));
        }

        for route in self.1.routes() {
            routes.push(route.map(Either3::B));
        }

        for route in self.2.routes() {
            routes.push(route.map(Either3::C));
        }

        routes
    }

    fn dispatch(&mut self,
                match_: Match<Self::Destination>,
                request: http::Request<()>)
        -> Self::Future
    {
        use self::Either3::*;

        let (destination, params) = match_.into_parts();

        match destination {
            A(d) => {
                let match_ = Match::new(d, params);
                A(self.0.dispatch(match_, request))
            }
            B(d) => {
                let match_ = Match::new(d, params);
                B(self.1.dispatch(match_, request))
            }
            C(d) => {
                let match_ = Match::new(d, params);
                C(self.2.dispatch(match_, request))
            }
        }
    }
}

impl<R0, R1, R2, U> Chain<U> for (R0, R1, R2) {
    type Resource = (R0, R1, R2, U);

    fn chain(self, other: U) -> Self::Resource {
        (self.0, self.1, self.2, other)
    }
}
// ===== 4 =====

#[derive(Clone)]
pub enum Either4<A = (), B = (), C = (), D = ()> {
    A(A),
    B(B),
    C(C),
    D(D),
}

impl<A, B, C, D> Future for Either4<A, B, C, D>
where
    A: Future,
    B: Future<Item = A::Item, Error = A::Error>,
    C: Future<Item = A::Item, Error = A::Error>,
    D: Future<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::Either4::*;

        match *self {
            A(ref mut f) => f.poll(),
            B(ref mut f) => f.poll(),
            C(ref mut f) => f.poll(),
            D(ref mut f) => f.poll(),
        }
    }
}

impl<R0, R1, R2, R3> Resource for (R0, R1, R2, R3)
where
    R0: Resource,
    R1: Resource<Body = R0::Body>,
    R2: Resource<Body = R0::Body>,
    R3: Resource<Body = R0::Body>,
{
    type Destination = Either4<R0::Destination, R1::Destination, R2::Destination, R3::Destination>;
    type Body = R0::Body;
    type Future = Either4<R0::Future, R1::Future, R2::Future, R3::Future>;

    fn routes(&self) -> RouteSet<Self::Destination> {
        let mut routes = RouteSet::new();

        for route in self.0.routes() {
            routes.push(route.map(Either4::A));
        }

        for route in self.1.routes() {
            routes.push(route.map(Either4::B));
        }

        for route in self.2.routes() {
            routes.push(route.map(Either4::C));
        }

        for route in self.3.routes() {
            routes.push(route.map(Either4::D));
        }

        routes
    }

    fn dispatch(&mut self,
                match_: Match<Self::Destination>,
                request: http::Request<()>)
        -> Self::Future
    {
        use self::Either4::*;

        let (destination, params) = match_.into_parts();

        match destination {
            A(d) => {
                let match_ = Match::new(d, params);
                A(self.0.dispatch(match_, request))
            }
            B(d) => {
                let match_ = Match::new(d, params);
                B(self.1.dispatch(match_, request))
            }
            C(d) => {
                let match_ = Match::new(d, params);
                C(self.2.dispatch(match_, request))
            }
            D(d) => {
                let match_ = Match::new(d, params);
                D(self.3.dispatch(match_, request))
            }
        }
    }
}

impl<R0, R1, R2, R3, U> Chain<U> for (R0, R1, R2, R3) {
    type Resource = (R0, R1, R2, R3, U);

    fn chain(self, other: U) -> Self::Resource {
        (self.0, self.1, self.2, self.3, other)
    }
}
// ===== 5 =====

#[derive(Clone)]
pub enum Either5<A = (), B = (), C = (), D = (), E = ()> {
    A(A),
    B(B),
    C(C),
    D(D),
    E(E),
}

impl<A, B, C, D, E> Future for Either5<A, B, C, D, E>
where
    A: Future,
    B: Future<Item = A::Item, Error = A::Error>,
    C: Future<Item = A::Item, Error = A::Error>,
    D: Future<Item = A::Item, Error = A::Error>,
    E: Future<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
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

impl<R0, R1, R2, R3, R4> Resource for (R0, R1, R2, R3, R4)
where
    R0: Resource,
    R1: Resource<Body = R0::Body>,
    R2: Resource<Body = R0::Body>,
    R3: Resource<Body = R0::Body>,
    R4: Resource<Body = R0::Body>,
{
    type Destination = Either5<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination>;
    type Body = R0::Body;
    type Future = Either5<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future>;

    fn routes(&self) -> RouteSet<Self::Destination> {
        let mut routes = RouteSet::new();

        for route in self.0.routes() {
            routes.push(route.map(Either5::A));
        }

        for route in self.1.routes() {
            routes.push(route.map(Either5::B));
        }

        for route in self.2.routes() {
            routes.push(route.map(Either5::C));
        }

        for route in self.3.routes() {
            routes.push(route.map(Either5::D));
        }

        for route in self.4.routes() {
            routes.push(route.map(Either5::E));
        }

        routes
    }

    fn dispatch(&mut self,
                match_: Match<Self::Destination>,
                request: http::Request<()>)
        -> Self::Future
    {
        use self::Either5::*;

        let (destination, params) = match_.into_parts();

        match destination {
            A(d) => {
                let match_ = Match::new(d, params);
                A(self.0.dispatch(match_, request))
            }
            B(d) => {
                let match_ = Match::new(d, params);
                B(self.1.dispatch(match_, request))
            }
            C(d) => {
                let match_ = Match::new(d, params);
                C(self.2.dispatch(match_, request))
            }
            D(d) => {
                let match_ = Match::new(d, params);
                D(self.3.dispatch(match_, request))
            }
            E(d) => {
                let match_ = Match::new(d, params);
                E(self.4.dispatch(match_, request))
            }
        }
    }
}

impl<R0, R1, R2, R3, R4, U> Chain<U> for (R0, R1, R2, R3, R4) {
    type Resource = (R0, R1, R2, R3, R4, U);

    fn chain(self, other: U) -> Self::Resource {
        (self.0, self.1, self.2, self.3, self.4, other)
    }
}
// ===== 6 =====

#[derive(Clone)]
pub enum Either6<A = (), B = (), C = (), D = (), E = (), F = ()> {
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
    B: Future<Item = A::Item, Error = A::Error>,
    C: Future<Item = A::Item, Error = A::Error>,
    D: Future<Item = A::Item, Error = A::Error>,
    E: Future<Item = A::Item, Error = A::Error>,
    F: Future<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
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

impl<R0, R1, R2, R3, R4, R5> Resource for (R0, R1, R2, R3, R4, R5)
where
    R0: Resource,
    R1: Resource<Body = R0::Body>,
    R2: Resource<Body = R0::Body>,
    R3: Resource<Body = R0::Body>,
    R4: Resource<Body = R0::Body>,
    R5: Resource<Body = R0::Body>,
{
    type Destination = Either6<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination, R5::Destination>;
    type Body = R0::Body;
    type Future = Either6<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future, R5::Future>;

    fn routes(&self) -> RouteSet<Self::Destination> {
        let mut routes = RouteSet::new();

        for route in self.0.routes() {
            routes.push(route.map(Either6::A));
        }

        for route in self.1.routes() {
            routes.push(route.map(Either6::B));
        }

        for route in self.2.routes() {
            routes.push(route.map(Either6::C));
        }

        for route in self.3.routes() {
            routes.push(route.map(Either6::D));
        }

        for route in self.4.routes() {
            routes.push(route.map(Either6::E));
        }

        for route in self.5.routes() {
            routes.push(route.map(Either6::F));
        }

        routes
    }

    fn dispatch(&mut self,
                match_: Match<Self::Destination>,
                request: http::Request<()>)
        -> Self::Future
    {
        use self::Either6::*;

        let (destination, params) = match_.into_parts();

        match destination {
            A(d) => {
                let match_ = Match::new(d, params);
                A(self.0.dispatch(match_, request))
            }
            B(d) => {
                let match_ = Match::new(d, params);
                B(self.1.dispatch(match_, request))
            }
            C(d) => {
                let match_ = Match::new(d, params);
                C(self.2.dispatch(match_, request))
            }
            D(d) => {
                let match_ = Match::new(d, params);
                D(self.3.dispatch(match_, request))
            }
            E(d) => {
                let match_ = Match::new(d, params);
                E(self.4.dispatch(match_, request))
            }
            F(d) => {
                let match_ = Match::new(d, params);
                F(self.5.dispatch(match_, request))
            }
        }
    }
}

impl<R0, R1, R2, R3, R4, R5, U> Chain<U> for (R0, R1, R2, R3, R4, R5) {
    type Resource = (R0, R1, R2, R3, R4, R5, U);

    fn chain(self, other: U) -> Self::Resource {
        (self.0, self.1, self.2, self.3, self.4, self.5, other)
    }
}
// ===== 7 =====

#[derive(Clone)]
pub enum Either7<A = (), B = (), C = (), D = (), E = (), F = (), G = ()> {
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
    B: Future<Item = A::Item, Error = A::Error>,
    C: Future<Item = A::Item, Error = A::Error>,
    D: Future<Item = A::Item, Error = A::Error>,
    E: Future<Item = A::Item, Error = A::Error>,
    F: Future<Item = A::Item, Error = A::Error>,
    G: Future<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
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

impl<R0, R1, R2, R3, R4, R5, R6> Resource for (R0, R1, R2, R3, R4, R5, R6)
where
    R0: Resource,
    R1: Resource<Body = R0::Body>,
    R2: Resource<Body = R0::Body>,
    R3: Resource<Body = R0::Body>,
    R4: Resource<Body = R0::Body>,
    R5: Resource<Body = R0::Body>,
    R6: Resource<Body = R0::Body>,
{
    type Destination = Either7<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination, R5::Destination, R6::Destination>;
    type Body = R0::Body;
    type Future = Either7<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future, R5::Future, R6::Future>;

    fn routes(&self) -> RouteSet<Self::Destination> {
        let mut routes = RouteSet::new();

        for route in self.0.routes() {
            routes.push(route.map(Either7::A));
        }

        for route in self.1.routes() {
            routes.push(route.map(Either7::B));
        }

        for route in self.2.routes() {
            routes.push(route.map(Either7::C));
        }

        for route in self.3.routes() {
            routes.push(route.map(Either7::D));
        }

        for route in self.4.routes() {
            routes.push(route.map(Either7::E));
        }

        for route in self.5.routes() {
            routes.push(route.map(Either7::F));
        }

        for route in self.6.routes() {
            routes.push(route.map(Either7::G));
        }

        routes
    }

    fn dispatch(&mut self,
                match_: Match<Self::Destination>,
                request: http::Request<()>)
        -> Self::Future
    {
        use self::Either7::*;

        let (destination, params) = match_.into_parts();

        match destination {
            A(d) => {
                let match_ = Match::new(d, params);
                A(self.0.dispatch(match_, request))
            }
            B(d) => {
                let match_ = Match::new(d, params);
                B(self.1.dispatch(match_, request))
            }
            C(d) => {
                let match_ = Match::new(d, params);
                C(self.2.dispatch(match_, request))
            }
            D(d) => {
                let match_ = Match::new(d, params);
                D(self.3.dispatch(match_, request))
            }
            E(d) => {
                let match_ = Match::new(d, params);
                E(self.4.dispatch(match_, request))
            }
            F(d) => {
                let match_ = Match::new(d, params);
                F(self.5.dispatch(match_, request))
            }
            G(d) => {
                let match_ = Match::new(d, params);
                G(self.6.dispatch(match_, request))
            }
        }
    }
}

impl<R0, R1, R2, R3, R4, R5, R6, U> Chain<U> for (R0, R1, R2, R3, R4, R5, R6) {
    type Resource = (R0, R1, R2, R3, R4, R5, R6, U);

    fn chain(self, other: U) -> Self::Resource {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, other)
    }
}
// ===== 8 =====

#[derive(Clone)]
pub enum Either8<A = (), B = (), C = (), D = (), E = (), F = (), G = (), H = ()> {
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
    B: Future<Item = A::Item, Error = A::Error>,
    C: Future<Item = A::Item, Error = A::Error>,
    D: Future<Item = A::Item, Error = A::Error>,
    E: Future<Item = A::Item, Error = A::Error>,
    F: Future<Item = A::Item, Error = A::Error>,
    G: Future<Item = A::Item, Error = A::Error>,
    H: Future<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
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

impl<R0, R1, R2, R3, R4, R5, R6, R7> Resource for (R0, R1, R2, R3, R4, R5, R6, R7)
where
    R0: Resource,
    R1: Resource<Body = R0::Body>,
    R2: Resource<Body = R0::Body>,
    R3: Resource<Body = R0::Body>,
    R4: Resource<Body = R0::Body>,
    R5: Resource<Body = R0::Body>,
    R6: Resource<Body = R0::Body>,
    R7: Resource<Body = R0::Body>,
{
    type Destination = Either8<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination, R5::Destination, R6::Destination, R7::Destination>;
    type Body = R0::Body;
    type Future = Either8<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future, R5::Future, R6::Future, R7::Future>;

    fn routes(&self) -> RouteSet<Self::Destination> {
        let mut routes = RouteSet::new();

        for route in self.0.routes() {
            routes.push(route.map(Either8::A));
        }

        for route in self.1.routes() {
            routes.push(route.map(Either8::B));
        }

        for route in self.2.routes() {
            routes.push(route.map(Either8::C));
        }

        for route in self.3.routes() {
            routes.push(route.map(Either8::D));
        }

        for route in self.4.routes() {
            routes.push(route.map(Either8::E));
        }

        for route in self.5.routes() {
            routes.push(route.map(Either8::F));
        }

        for route in self.6.routes() {
            routes.push(route.map(Either8::G));
        }

        for route in self.7.routes() {
            routes.push(route.map(Either8::H));
        }

        routes
    }

    fn dispatch(&mut self,
                match_: Match<Self::Destination>,
                request: http::Request<()>)
        -> Self::Future
    {
        use self::Either8::*;

        let (destination, params) = match_.into_parts();

        match destination {
            A(d) => {
                let match_ = Match::new(d, params);
                A(self.0.dispatch(match_, request))
            }
            B(d) => {
                let match_ = Match::new(d, params);
                B(self.1.dispatch(match_, request))
            }
            C(d) => {
                let match_ = Match::new(d, params);
                C(self.2.dispatch(match_, request))
            }
            D(d) => {
                let match_ = Match::new(d, params);
                D(self.3.dispatch(match_, request))
            }
            E(d) => {
                let match_ = Match::new(d, params);
                E(self.4.dispatch(match_, request))
            }
            F(d) => {
                let match_ = Match::new(d, params);
                F(self.5.dispatch(match_, request))
            }
            G(d) => {
                let match_ = Match::new(d, params);
                G(self.6.dispatch(match_, request))
            }
            H(d) => {
                let match_ = Match::new(d, params);
                H(self.7.dispatch(match_, request))
            }
        }
    }
}

impl<R0, R1, R2, R3, R4, R5, R6, R7, U> Chain<U> for (R0, R1, R2, R3, R4, R5, R6, R7) {
    type Resource = (R0, R1, R2, R3, R4, R5, R6, R7, U);

    fn chain(self, other: U) -> Self::Resource {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, other)
    }
}
// ===== 9 =====

#[derive(Clone)]
pub enum Either9<A = (), B = (), C = (), D = (), E = (), F = (), G = (), H = (), I = ()> {
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
    B: Future<Item = A::Item, Error = A::Error>,
    C: Future<Item = A::Item, Error = A::Error>,
    D: Future<Item = A::Item, Error = A::Error>,
    E: Future<Item = A::Item, Error = A::Error>,
    F: Future<Item = A::Item, Error = A::Error>,
    G: Future<Item = A::Item, Error = A::Error>,
    H: Future<Item = A::Item, Error = A::Error>,
    I: Future<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
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

impl<R0, R1, R2, R3, R4, R5, R6, R7, R8> Resource for (R0, R1, R2, R3, R4, R5, R6, R7, R8)
where
    R0: Resource,
    R1: Resource<Body = R0::Body>,
    R2: Resource<Body = R0::Body>,
    R3: Resource<Body = R0::Body>,
    R4: Resource<Body = R0::Body>,
    R5: Resource<Body = R0::Body>,
    R6: Resource<Body = R0::Body>,
    R7: Resource<Body = R0::Body>,
    R8: Resource<Body = R0::Body>,
{
    type Destination = Either9<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination, R5::Destination, R6::Destination, R7::Destination, R8::Destination>;
    type Body = R0::Body;
    type Future = Either9<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future, R5::Future, R6::Future, R7::Future, R8::Future>;

    fn routes(&self) -> RouteSet<Self::Destination> {
        let mut routes = RouteSet::new();

        for route in self.0.routes() {
            routes.push(route.map(Either9::A));
        }

        for route in self.1.routes() {
            routes.push(route.map(Either9::B));
        }

        for route in self.2.routes() {
            routes.push(route.map(Either9::C));
        }

        for route in self.3.routes() {
            routes.push(route.map(Either9::D));
        }

        for route in self.4.routes() {
            routes.push(route.map(Either9::E));
        }

        for route in self.5.routes() {
            routes.push(route.map(Either9::F));
        }

        for route in self.6.routes() {
            routes.push(route.map(Either9::G));
        }

        for route in self.7.routes() {
            routes.push(route.map(Either9::H));
        }

        for route in self.8.routes() {
            routes.push(route.map(Either9::I));
        }

        routes
    }

    fn dispatch(&mut self,
                match_: Match<Self::Destination>,
                request: http::Request<()>)
        -> Self::Future
    {
        use self::Either9::*;

        let (destination, params) = match_.into_parts();

        match destination {
            A(d) => {
                let match_ = Match::new(d, params);
                A(self.0.dispatch(match_, request))
            }
            B(d) => {
                let match_ = Match::new(d, params);
                B(self.1.dispatch(match_, request))
            }
            C(d) => {
                let match_ = Match::new(d, params);
                C(self.2.dispatch(match_, request))
            }
            D(d) => {
                let match_ = Match::new(d, params);
                D(self.3.dispatch(match_, request))
            }
            E(d) => {
                let match_ = Match::new(d, params);
                E(self.4.dispatch(match_, request))
            }
            F(d) => {
                let match_ = Match::new(d, params);
                F(self.5.dispatch(match_, request))
            }
            G(d) => {
                let match_ = Match::new(d, params);
                G(self.6.dispatch(match_, request))
            }
            H(d) => {
                let match_ = Match::new(d, params);
                H(self.7.dispatch(match_, request))
            }
            I(d) => {
                let match_ = Match::new(d, params);
                I(self.8.dispatch(match_, request))
            }
        }
    }
}

impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, U> Chain<U> for (R0, R1, R2, R3, R4, R5, R6, R7, R8) {
    type Resource = (R0, R1, R2, R3, R4, R5, R6, R7, R8, U);

    fn chain(self, other: U) -> Self::Resource {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, other)
    }
}
// ===== 10 =====

#[derive(Clone)]
pub enum Either10<A = (), B = (), C = (), D = (), E = (), F = (), G = (), H = (), I = (), J = ()> {
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
    B: Future<Item = A::Item, Error = A::Error>,
    C: Future<Item = A::Item, Error = A::Error>,
    D: Future<Item = A::Item, Error = A::Error>,
    E: Future<Item = A::Item, Error = A::Error>,
    F: Future<Item = A::Item, Error = A::Error>,
    G: Future<Item = A::Item, Error = A::Error>,
    H: Future<Item = A::Item, Error = A::Error>,
    I: Future<Item = A::Item, Error = A::Error>,
    J: Future<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
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

impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9> Resource for (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9)
where
    R0: Resource,
    R1: Resource<Body = R0::Body>,
    R2: Resource<Body = R0::Body>,
    R3: Resource<Body = R0::Body>,
    R4: Resource<Body = R0::Body>,
    R5: Resource<Body = R0::Body>,
    R6: Resource<Body = R0::Body>,
    R7: Resource<Body = R0::Body>,
    R8: Resource<Body = R0::Body>,
    R9: Resource<Body = R0::Body>,
{
    type Destination = Either10<R0::Destination, R1::Destination, R2::Destination, R3::Destination, R4::Destination, R5::Destination, R6::Destination, R7::Destination, R8::Destination, R9::Destination>;
    type Body = R0::Body;
    type Future = Either10<R0::Future, R1::Future, R2::Future, R3::Future, R4::Future, R5::Future, R6::Future, R7::Future, R8::Future, R9::Future>;

    fn routes(&self) -> RouteSet<Self::Destination> {
        let mut routes = RouteSet::new();

        for route in self.0.routes() {
            routes.push(route.map(Either10::A));
        }

        for route in self.1.routes() {
            routes.push(route.map(Either10::B));
        }

        for route in self.2.routes() {
            routes.push(route.map(Either10::C));
        }

        for route in self.3.routes() {
            routes.push(route.map(Either10::D));
        }

        for route in self.4.routes() {
            routes.push(route.map(Either10::E));
        }

        for route in self.5.routes() {
            routes.push(route.map(Either10::F));
        }

        for route in self.6.routes() {
            routes.push(route.map(Either10::G));
        }

        for route in self.7.routes() {
            routes.push(route.map(Either10::H));
        }

        for route in self.8.routes() {
            routes.push(route.map(Either10::I));
        }

        for route in self.9.routes() {
            routes.push(route.map(Either10::J));
        }

        routes
    }

    fn dispatch(&mut self,
                match_: Match<Self::Destination>,
                request: http::Request<()>)
        -> Self::Future
    {
        use self::Either10::*;

        let (destination, params) = match_.into_parts();

        match destination {
            A(d) => {
                let match_ = Match::new(d, params);
                A(self.0.dispatch(match_, request))
            }
            B(d) => {
                let match_ = Match::new(d, params);
                B(self.1.dispatch(match_, request))
            }
            C(d) => {
                let match_ = Match::new(d, params);
                C(self.2.dispatch(match_, request))
            }
            D(d) => {
                let match_ = Match::new(d, params);
                D(self.3.dispatch(match_, request))
            }
            E(d) => {
                let match_ = Match::new(d, params);
                E(self.4.dispatch(match_, request))
            }
            F(d) => {
                let match_ = Match::new(d, params);
                F(self.5.dispatch(match_, request))
            }
            G(d) => {
                let match_ = Match::new(d, params);
                G(self.6.dispatch(match_, request))
            }
            H(d) => {
                let match_ = Match::new(d, params);
                H(self.7.dispatch(match_, request))
            }
            I(d) => {
                let match_ = Match::new(d, params);
                I(self.8.dispatch(match_, request))
            }
            J(d) => {
                let match_ = Match::new(d, params);
                J(self.9.dispatch(match_, request))
            }
        }
    }
}

impl<R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, U> Chain<U> for (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9) {
    type Resource = (R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, U);

    fn chain(self, other: U) -> Self::Resource {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, other)
    }
}
