use super::{Context, Response, Serializer};
use futures::future::Either;
use http;
use util::tuple::Either2;
use util::BufStream;

impl<A, B> Response for Either<A, B>
where
    A: Response,
    B: Response,
{
    type Buf = <Self::Body as BufStream>::Item;
    type Body = Either2<<A as Response>::Body, <B as Response>::Body>;

    fn into_http<S: Serializer>(self, context: &Context<S>) -> http::Response<Self::Body> {
        match self {
            Either::A(a) => Either2::A::<A, B>(a).into_http(context),
            Either::B(b) => Either2::B::<A, B>(b).into_http(context),
        }
    }
}
