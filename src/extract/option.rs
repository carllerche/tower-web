//! Types used to extract `Option` values from an HTTP request.

use crate::extract::{Extract, ExtractFuture, Error, Context};
use crate::util::BufStream;

use futures::{Async, Poll};

/// Extract an `Option` value from an HTTP request.
#[derive(Debug)]
pub struct ExtractOptionFuture<T> {
    inner: T,
    none: bool,
}

impl<T, B: BufStream> Extract<B> for Option<T>
where T: Extract<B>,
{
    type Future = ExtractOptionFuture<T::Future>;

    fn extract(ctx: &Context) -> Self::Future {
        ExtractOptionFuture {
            inner: T::extract(ctx),
            none: false,
        }
    }
}

impl<T> ExtractFuture for ExtractOptionFuture<T>
where T: ExtractFuture,
{
    type Item = Option<T::Item>;

    fn poll(&mut self) -> Poll<(), Error> {
        match self.inner.poll() {
            Err(ref e) if e.is_missing_argument() => {
                self.none = true;
                Ok(Async::Ready(()))
            }
            res => res,
        }
    }

    fn extract(self) -> Self::Item {
        if self.none {
            None
        } else {
            Some(self.inner.extract())
        }
    }
}
