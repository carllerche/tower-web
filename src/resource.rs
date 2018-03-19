use routing::{RouteSet, Match};

use bytes::Bytes;
use futures::{Future, Stream};
use http;

/// A resource
pub trait Resource: Clone + Send + 'static {
    /// Identifies a route.
    type Destination: Clone + Send + Sync + 'static;

    /// The HTTP response body type.
    type Body: Stream<Item = Bytes, Error = ::Error> + Send + 'static;

    /// Response future
    type Future: Future<Item = http::Response<Self::Body>, Error = ::Error> + Send + 'static;

    /// Return the routes associated with the resource.
    fn routes(&self) -> RouteSet<Self::Destination>;

    fn dispatch(&mut self,
                route: Match<Self::Destination>,
                request: http::Request<()>) -> Self::Future;
}

/// Combine two resources
pub trait Chain<U> {
    type Resource;

    fn chain(self, other: U) -> Self::Resource;
}

pub mod tuple {
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
    where A: Future,
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

    impl<R1, R2> Resource for (R1, R2)
    where R1: Resource,
          R2: Resource<Body = R1::Body>,
    {
        type Destination = Either2<R1::Destination, R2::Destination>;
        type Body = R1::Body;
        type Future = Either2<R1::Future, R2::Future>;

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

            let (destination, condition) = match_.into_parts();

            match destination {
                A(d) => {
                    let match_ = Match::new(d, condition);
                    A(self.0.dispatch(match_, request))
                }
                B(d) => {
                    let match_ = Match::new(d, condition);
                    B(self.1.dispatch(match_, request))
                }
            }
        }
    }

    impl<R1, R2, U> Chain<U> for (R1, R2) {
        type Resource = (R1, R2, U);

        fn chain(self, other: U) -> Self::Resource {
            (self.0, self.1, other)
        }
    }
}
