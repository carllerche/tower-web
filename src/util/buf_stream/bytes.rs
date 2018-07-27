use super::BufStream;

use bytes::Bytes;
use futures::Poll;

use std::io;

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
