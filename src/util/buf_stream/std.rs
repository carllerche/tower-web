use util::BufStream;

use bytes::Buf;
use futures::{Stream, Poll};

pub struct StdStream<T>(T);

impl<T> StdStream<T> {
    pub fn new(stream: T) -> StdStream<T> {
        StdStream(stream)
    }
}

impl<T> BufStream for StdStream<T>
where T: Stream,
      T::Item: Buf,
{
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.0.poll()
    }
}
