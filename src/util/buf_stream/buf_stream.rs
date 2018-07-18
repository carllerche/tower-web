use super::{Chain, Collect, FromBufStream, SizeHint};

use bytes::{Buf, Bytes};
use futures::Poll;

use std::io;

pub trait BufStream {
    type Item: Buf;
    type Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error>;

    fn size_hint(&self) -> SizeHint {
        SizeHint::default()
    }

    fn chain<T>(self, other: T) -> Chain<Self, T>
    where
        Self: Sized,
        T: BufStream<Error = Self::Error>,
    {
        Chain::new(self, other)
    }

    fn collect<T>(self) -> Collect<Self, T>
    where
        Self: Sized,
        T: FromBufStream,
    {
        Collect::new(self)
    }
}

impl BufStream for String {
    type Item = io::Cursor<Vec<u8>>;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use std::mem;

        if self.is_empty() {
            return Ok(None.into());
        }

        let bytes = mem::replace(self, String::new()).into_bytes();
        let buf = io::Cursor::new(bytes);

        Ok(Some(buf).into())
    }
}

impl BufStream for &'static str {
    type Item = io::Cursor<&'static [u8]>;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use std::mem;

        if self.is_empty() {
            return Ok(None.into());
        }

        let bytes = mem::replace(self, "").as_bytes();
        let buf = io::Cursor::new(bytes);

        Ok(Some(buf).into())
    }
}

impl BufStream for Bytes {
    type Item = io::Cursor<Bytes>;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        use std::mem;

        if self.is_empty() {
            return Ok(None.into());
        }

        let bytes = mem::replace(self, Bytes::new());
        let buf = io::Cursor::new(bytes);

        Ok(Some(buf).into())
    }
}
