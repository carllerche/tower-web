use error::Error;
use util::BufStream;

use bytes::Buf;
use futures::Future;
use http::{Request, Response};

/// Handles an error
pub trait Catch {

    type Buf: Buf;

    type Body: BufStream<Item = Self::Buf, Error = Error>;

    type Future: Future<Item = Response<Self::Body>, Error = Error>;

    /// Handles an error
    fn catch(&mut self, request: &Request<()>, error: Error) -> Self::Future;
}

pub trait IntoCatch {
    type Catch: Catch;

    fn into_catch(self) -> Self::Catch;
}
