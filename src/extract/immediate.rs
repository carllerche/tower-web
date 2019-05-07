use crate::extract::{ExtractFuture, Error};

use futures::{Poll};

/// Implements `ExtractFuture` such that the result is immediately available.
///
/// This type is useful when implementing `Extract` for types that do not
/// require any asynchronous processing. For example, extracting an HTTP header
/// value from an HTTP request can complete immediately as all the information
/// is present.
#[derive(Debug)]
pub struct Immediate<T> {
    inner: Result<T, Option<Error>>,
}

impl<T> Immediate<T> {
    /// Create a new `Immediate` instance from a `Result` value.
    ///
    /// When polling the returned `Immediate` instance, it will yield `result`.
    pub fn result(result: Result<T, Error>) -> Immediate<T> {
        Immediate {
            inner: result.map_err(Some),
        }
    }

    /// Create a new `Immediate` instance that is in the success state.
    ///
    /// When polling the returned `Immediate` instance, it will yield `value`.
    pub fn ok(value: T) -> Immediate<T> {
        Immediate::result(Ok(value))
    }

    /// Create a new `Immediate` instance that is in the error state.
    ///
    /// When polling the returned `Immediate` instance, it will yield `error`.
    pub fn err(error: Error) -> Immediate<T> {
        Immediate::result(Err(error))
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
