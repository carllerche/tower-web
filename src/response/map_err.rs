use error::ErrorKind;
use util::BufStream;

use futures::{Future, Poll};

// TODO: Move this into `error` mod
#[derive(Debug)]
pub struct MapErr<T> {
    inner: State<T>,
}

#[derive(Debug)]
enum State<T> {
    Inner(T),
    Immediate(Option<::Error>),
}

impl<T> MapErr<T> {
    pub fn new(inner: T) -> MapErr<T> {
        MapErr {
            inner: State::Inner(inner),
        }
    }

    pub fn immediate(error: ::Error) -> MapErr<T> {
        MapErr {
            inner: State::Immediate(Some(error)),
        }
    }
}

impl<T: Future> Future for MapErr<T> {
    type Item = T::Item;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::State::*;

        match self.inner {
            Inner(ref mut f) => f.poll().map_err(|_| ErrorKind::internal().into()),
            Immediate(ref mut e) => Err(e.take().unwrap()),
        }
    }
}

impl<T: BufStream> BufStream for MapErr<T> {
    type Item = T::Item;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use self::State::*;

        match self.inner {
            Inner(ref mut f) => f.poll().map_err(|_| ErrorKind::internal().into()),
            Immediate(ref mut e) => Err(e.take().unwrap()),
        }
    }
}
