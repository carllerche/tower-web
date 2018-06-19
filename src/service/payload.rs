use bytes::Buf;
use futures::{Poll, Stream};

pub trait Payload {
    type Data: Buf;

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, ::Error>;
}

impl<T> Payload for T
where T: Stream<Error = ::Error>,
      T::Item: Buf,
{
    type Data = T::Item;

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, ::Error> {
        self.poll()
    }
}
