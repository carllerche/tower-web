use super::BufStream;

use bytes::BytesMut;
use futures::{Async, Poll};
use tokio_fs::File;
use tokio_io::AsyncRead;

use std::io;

impl BufStream for File {
    type Item = io::Cursor<BytesMut>;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let mut v = BytesMut::new();

        let len = try_ready!(self.read_buf(&mut v));

        if len == 0 {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::Ready(Some(io::Cursor::new(v))))
        }
    }
}
