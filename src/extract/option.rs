use extract::{Extract, ExtractFuture, Error, Context};

use futures::{Async, Poll};

pub struct ExtractOptionFuture<T> {
    inner: T,
    none: bool,
}

impl<T> Extract for Option<T>
where T: Extract,
{
    type Future = ExtractOptionFuture<T::Future>;

    fn into_future(ctx: &Context) -> Self::Future {
        ExtractOptionFuture {
            inner: T::into_future(ctx),
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
