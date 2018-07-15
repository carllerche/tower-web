use extract::{ExtractFuture, Error};

use futures::{Poll};

pub struct Immediate<T> {
    inner: Result<T, Option<Error>>,
}

impl<T> Immediate<T> {
    pub fn ok(value: T) -> Immediate<T> {
        Immediate { inner: Ok(value) }
    }

    pub fn error(error: Error) -> Immediate<T> {
        Immediate { inner: Err(Some(error)) }
    }
}

impl<T> ExtractFuture for Immediate<T> {
    type Item = T;

    fn poll(&mut self) -> Poll<(), Error> {
        match self.inner {
            Ok(_) => Ok(().into()),
            Err(ref mut err) => {
                Err(err.take().unwrap())
            }
        }
    }

    fn extract(self) -> T {
        self.inner.unwrap()
    }
}

impl<T, E> From<Result<T, E>> for Immediate<T>
where E: Into<Error>,
{
    fn from(src: Result<T, E>) -> Self {
        let inner = src.map_err(|e| Some(e.into()));
        Immediate { inner }
    }
}
