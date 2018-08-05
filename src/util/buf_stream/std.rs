use util::BufStream;

use bytes::Buf;
use futures::{Poll, Stream, stream};

#[derive(Default)]
pub struct StdStream<T>(T);

impl<T> StdStream<T> {
    pub fn new(stream: T) -> StdStream<T> {
        StdStream(stream)
    }
}

impl<T> BufStream for StdStream<T>
where
    T: Stream,
    T::Item: Buf,
{
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.0.poll()
    }
}

pub type Empty<Item, Error> = StdStream<stream::Empty<Item, Error>>;
pub fn empty<Item, Error>() -> Empty<Item, Error> { StdStream(stream::empty()) }
