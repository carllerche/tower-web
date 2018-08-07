use error::ErrorKind;
use util::BufStream;

use futures::{Future, Poll};

#[derive(Debug)]
pub struct Map<T> {
    inner: State<T>,
}

#[derive(Debug)]
enum State<T> {
    Inner(T),
    Immediate(Option<::Error>),
}

impl<T> Map<T> {
    pub fn new(inner: T) -> Map<T> {
        Map {
            inner: State::Inner(inner),
        }
    }

    pub fn immediate(error: ::Error) -> Map<T> {
        Map {
            inner: State::Immediate(Some(error)),
        }
    }
}

impl<T: Future> Future for Map<T> {
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

impl<T: BufStream> BufStream for Map<T> {
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
