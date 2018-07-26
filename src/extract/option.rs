use extract::{Extract, ExtractFuture, Error, Context};
use util::BufStream;

use futures::{Async, Poll};

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
            Err(ref e) if e.is_missing_param() => {
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
