use super::{BufStream, SizeHint};
use futures::{future::Either, Poll};

impl<A, B> BufStream for Either<A, B>
where
    A: BufStream,
    B: BufStream<Item = A::Item, Error = A::Error>,
{
    type Item = A::Item;
    type Error = A::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self {
            Either::A(a) => a.poll(),
            Either::B(b) => b.poll(),
        }
    }

    fn size_hint(&self) -> SizeHint {
        match self {
            Either::A(a) => a.size_hint(),
            Either::B(b) => b.size_hint(),
        }
    }
}
