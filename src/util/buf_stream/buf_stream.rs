use super::{Chain, Collect, FromBufStream, SizeHint};

use bytes::Buf;
use futures::{Async, Poll};

/// An asynchronous stream of bytes.
///
/// `BufStream` asynchronously yields values implementing `Buf`, i.e. byte
/// buffers.
pub trait BufStream {
    /// Values yielded by the `BufStream`.
    type Item: Buf;

    /// The error type this `BufStream` might generate.
    type Error;

    /// Attempt to pull out the next buffer of this stream, registering the
    /// current task for wakeup if the value is not yet available, and returning
    /// `None` if the stream is exhausted.
    ///
    /// # Return value
    ///
    /// There are several possible return values, each indicating a distinct
    /// stream state:
    ///
    /// - `Ok(Async::NotReady)` means that this stream's next value is not ready
    /// yet. Implementations will ensure that the current task will be notified
    /// when the next value may be ready.
    ///
    /// - `Ok(Async::Ready(Some(buf)))` means that the stream has successfully
    /// produced a value, `buf`, and may produce further values on subsequent
    /// `poll` calls.
    ///
    /// - `Ok(Async::Ready(None))` means that the stream has terminated, and
    /// `poll` should not be invoked again.
    ///
    /// # Panics
    ///
    /// Once a stream is finished, i.e. `Ready(None)` has been returned, further
    /// calls to `poll` may result in a panic or other "bad behavior".
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error>;

    /// Returns the bounds on the remaining length of the iterator.
    ///
    /// # Implementation notes
    ///
    /// It is not enforced that a `BufStreaam` yields the declared amount of
    /// data. A buggy implementation may yield less than the lower bound or more
    /// than the upper bound.
    ///
    /// `size_hint()` is primarily intended to be used for optimizations such as
    /// reserving space for the data, but must not be trusted to e.g. omit
    /// bounds checks in unsafe code. An incorrect implemeentation of
    /// `size_hint()` should not lead to memory safety violations.
    ///
    /// That said, the implementation should provide a correct estimation,
    /// because otherwise it would be a violation of the trait's protocol.
    fn size_hint(&self) -> SizeHint {
        SizeHint::default()
    }

    /// Takes two buf streams and creates a new buf stream over both in
    /// sequence.
    ///
    /// `chain()` will return a new `BufStream` value which will first yield all
    /// data from `self` then all data from `other`.
    ///
    /// In other words, it links two buf streams together, in a chain.
    fn chain<T>(self, other: T) -> Chain<Self, T>
    where
        Self: Sized,
        T: BufStream<Error = Self::Error>,
    {
        Chain::new(self, other)
    }

    /// Consumes all data from `self`, storing it in byte storage of type `T`.
    fn collect<T>(self) -> Collect<Self, T>
    where
        Self: Sized,
        T: FromBufStream,
    {
        Collect::new(self)
    }
}

impl<B> BufStream for Option<B>
where
    B: BufStream,
{
    type Item = B::Item;
    type Error = B::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self {
            Some(b) => b.poll(),
            None => Ok(Async::Ready(None)),
        }
    }

    fn size_hint(&self) -> SizeHint {
        match self {
            Some(b) => b.size_hint(),
            None => SizeHint::default(),
        }
    }
}
