use crate::error::Never;
use super::BufStream;

use futures::Poll;

use std::io;
use std::mem;

impl BufStream for String {
    type Item = io::Cursor<Vec<u8>>;
    type Error = Never;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
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
    type Error = Never;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.is_empty() {
            return Ok(None.into());
        }

        let bytes = mem::replace(self, "").as_bytes();
        let buf = io::Cursor::new(bytes);

        Ok(Some(buf).into())
    }
}
