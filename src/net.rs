//! Networking types and trait

use futures::{Stream, Poll};
use tokio::net::TcpStream;
use tokio_io::{AsyncRead, AsyncWrite};

use std::io;
use std::net::SocketAddr;

#[cfg(feature = "rustls")]
use tokio_rustls::{TlsStream, rustls::ServerSession};

/// A stream between a local and remote target.
pub trait Connection: AsyncRead + AsyncWrite {
    /// Returns the socket address of the remote peer of this connection.
    fn peer_addr(&self) -> Option<SocketAddr>;
}

/// An asynchronous stream of connections.
pub trait ConnectionStream {
    /// Connection type yielded each iteration.
    type Item: Connection;

    /// Attempt to resolve the next connection, registering the current task for
    /// wakeup if one is not yet available.
    fn poll_next(&mut self) -> Poll<Option<Self::Item>, io::Error>;
}

impl Connection for TcpStream {
    fn peer_addr(&self) -> Option<SocketAddr> {
        TcpStream::peer_addr(self).ok()
    }
}

#[cfg(feature = "rustls")]
impl Connection for TlsStream<TcpStream, ServerSession> {
    fn peer_addr(&self) -> Option<SocketAddr> {
        TcpStream::peer_addr(self.get_ref().0).ok()
    }
}

impl<T> ConnectionStream for T
where
    T: Stream<Error = io::Error>,
    T::Item: Connection,
{
    type Item = <Self as Stream>::Item;

    fn poll_next(&mut self) -> Poll<Option<Self::Item>, io::Error> {
        self.poll()
    }
}

#[derive(Debug)]
pub(crate) struct Lift<T>(pub(crate) T);

impl<T: ConnectionStream> Stream for Lift<T> {
    type Item = <T as ConnectionStream>::Item;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.0.poll_next()
    }
}
