use crate::util::BufStream;

use bytes::Buf;
use futures::{Poll, Stream};

/// Wraps a `futures::Stream` that yields `Buf` values and provides a
/// `BufStream` implementation.
#[derive(Debug)]
pub struct StdStream<T>(T);

impl<T> StdStream<T> {
    /// Create a new `StdStream` containing `stream`.
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
