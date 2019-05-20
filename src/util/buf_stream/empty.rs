use bytes::Buf;
use futures::{Async, Poll};
use std::marker::PhantomData;
use crate::util::buf_stream::BufStream;

/// A `BufStream` that contains no data.
#[derive(Debug, Copy, Clone)]
pub struct Empty<Item, Error>(PhantomData<(Item, Error)>);

/// Create a new `Empty` instance.
pub fn empty<Item, Error>() -> Empty<Item, Error> {
    Default::default()
}

impl<Item, Error> Empty<Item, Error> {
    /// Create a new `Empty` instance.
    pub fn new() -> Self {
        Default::default()
    }
}

impl<Item, Error> Default for Empty<Item, Error> {
    fn default() -> Self {
        Empty(Default::default())
    }
}

impl<Item, Error> BufStream for Empty<Item, Error>
where
    Item: Buf,
{
    type Item = Item;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(Async::Ready(None))
    }
}
