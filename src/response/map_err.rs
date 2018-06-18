use futures::{Future, Poll};

pub struct MapErr<T> {
    inner: T,
}

impl<T> MapErr<T> {
    pub fn new(inner: T) -> MapErr<T> {
        MapErr { inner }
    }
}

impl<T: Future> Future for MapErr<T> {
    type Item = T::Item;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll().map_err(|_| ::Error::Internal)
    }
}
