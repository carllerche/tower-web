use super::{Chain, Collect, FromBufStream, SizeHint};

use bytes::Buf;
use futures::{Poll, Async};

pub trait BufStream {
    type Item: Buf;
    type Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error>;

    fn size_hint(&self) -> SizeHint {
        SizeHint::default()
    }

    fn chain<T>(self, other: T) -> Chain<Self, T>
    where
        Self: Sized,
        T: BufStream<Error = Self::Error>,
    {
        Chain::new(self, other)
    }

    fn collect<T>(self) -> Collect<Self, T>
    where
        Self: Sized,
        T: FromBufStream,
    {
        Collect::new(self)
    }
}

impl<B> BufStream for Option<B>
where
    B: BufStream,
{
    type Item = B::Item;
    type Error = B::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self {
            Some(b) => b.poll(),
            None => Ok(Async::Ready(None)),
        }
    }

    fn size_hint(&self) -> SizeHint {
        match self {
            Some(b) => b.size_hint(),
            None => SizeHint::default(),
        }
    }
}
