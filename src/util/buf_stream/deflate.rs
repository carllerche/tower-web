//! TODO: Dox

use super::{BufStream, SizeHint};

use bytes::{BytesMut, Bytes, Buf, BufMut};
use flate2::{Compress, Compression, FlushCompress, Status};
use futures::Poll;
use futures::Async::*;

use std::cmp;
use std::io;

/// Compress a buf stream using zlib deflate.
#[derive(Debug)]
pub struct CompressStream<T> {
    // The inner BufStream
    inner: T,

    // `true` when the inner buffer returned `None`
    inner_eof: bool,

    // `true` when the deflated stream is complete
    eof: bool,

    // Buffers input
    src_buf: BytesMut,

    // Buffers output
    dst_buf: BytesMut,

    // Compression
    compress: Compress,
}

/// Errors returned by `CompressStream`.
#[derive(Debug)]
pub struct Error<T> {
    /// `None` represents a deflate error
    inner: Option<T>,
}

const MIN_BUF: usize = 1024;

impl<T> CompressStream<T>
where T: BufStream,
{
    /// Create a new `CompressStream` which returns the compressed data from
    /// `inner`.
    ///
    /// `level` specifies the compression level.
    pub fn new(inner: T, level: Compression) -> CompressStream<T> {
        CompressStream {
            inner,
            inner_eof: false,
            eof: false,
            src_buf: BytesMut::new(),
            dst_buf: BytesMut::new(),
            compress: Compress::new(level, false),
        }
    }
}

impl<T> BufStream for CompressStream<T>
where T: BufStream,
{
    type Item = io::Cursor<Bytes>;
    type Error = Error<T::Error>;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            if self.eof {
                return Ok(Ready(None));
            }

            // First, if needed, try filling the buffer
            if !self.inner_eof {
                let res = self.inner.poll()
                    .map_err(|e| {
                        Error { inner: Some(e) }
                    });

                match try_ready!(res) {
                    Some(buf) => {
                        self.src_buf.reserve(buf.remaining());
                        self.src_buf.put(buf);
                    }
                    None => {
                        self.inner_eof = true;
                    }
                }
            }

            let before_out = self.compress.total_out();
            let before_in = self.compress.total_in();

            let flush = if self.inner_eof {
                FlushCompress::Finish
            } else {
                FlushCompress::None
            };

            // Ensure the destination buffer has capacity
            let amt = cmp::max(self.src_buf.len() / 2, MIN_BUF);
            self.dst_buf.reserve(amt);

            let ret = unsafe {
                self.compress.compress(
                    &self.src_buf,
                    self.dst_buf.bytes_mut(),
                    flush)
            };

            let written = (self.compress.total_out() - before_out) as usize;
            let consumed = (self.compress.total_in() - before_in) as usize;

            unsafe { self.dst_buf.advance_mut(written); }
            self.src_buf.split_to(consumed);

            match ret {
                // If we haven't ready any data and we haven't hit EOF yet,
                // then we need to keep asking for more data because if we
                // return that 0 bytes of data have been read then it will
                // be interpreted as EOF.
                Ok(Status::Ok) | Ok(Status::BufError) if written == 0 && !self.inner_eof => {
                    continue
                }
                Ok(Status::Ok) | Ok(Status::BufError) => {
                    break;
                }
                Ok(Status::StreamEnd) => {
                    self.eof = true;
                    break;
                }
                Err(..) => {
                    return Err(Error { inner: None });
                }
            }
        }

        let buf = io::Cursor::new(self.dst_buf.take().freeze());
        return Ok(Ready(Some(buf)));
    }

    fn size_hint(&self) -> SizeHint {
        // TODO: How should this work?
        self.inner.size_hint()
    }
}
