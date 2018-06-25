use super::{
    Chain,
    Collect,
    FromBufStream,
    SizeHint,
};

use bytes::Buf;
use futures::{Poll, Stream};

use std::io;

pub trait BufStream {
    type Item: Buf;
    type Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error>;

    fn size_hint(&self) -> SizeHint {
        unimplemented!();
    }

    fn chain<T>(self, other: T) -> Chain<Self, T>
    where Self: Sized,
          T: BufStream,
    {
        Chain::new(self, other)
    }

    fn collect<T>(self) -> Collect<Self, T>
    where Self: Sized,
          T: FromBufStream,
    {
        Collect::new(self)
    }
}

impl BufStream for String {
    type Item = io::Cursor<Vec<u8>>;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        unimplemented!();
    }
}
