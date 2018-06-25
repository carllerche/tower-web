use super::{BufStream, FromBufStream};

use futures::{Future, Poll};

use std::marker::PhantomData;

pub struct Collect<T, U> {
    stream: T,
    buffer: Option<U>,
}

impl<T, U> Collect<T, U>
where T: BufStream,
      U: FromBufStream,
{
    pub(crate) fn new(stream: T) -> Collect<T, U> {
        let buffer = U::with_capacity(&stream.size_hint());

        Collect {
            stream,
            buffer: Some(buffer),
        }
    }
}

impl<T, U> Future for Collect<T, U>
where T: BufStream,
      U: FromBufStream,
{
    type Item = U;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            match try_ready!(self.stream.poll()) {
                Some(mut buf) => {
                    self.buffer.as_mut().expect("cannot poll after done")
                        .extend(&mut buf);
                }
                None => {
                    let val = self.buffer.take().expect("cannot poll after done");
                    return Ok(val.into());
                }
            }
        }
    }
}
