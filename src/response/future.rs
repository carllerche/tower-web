use response::Response;

use futures::{Future, Poll};
use http;

// TODO: Move this into codegen?

#[derive(Debug)]
pub struct ResponseFuture<T, S> {
    future: T,
    serializer: S,
    // content_type: S::ContentType,
}

/*
 * State strategy:
 *
 * a) async - Extract args
 * b) sync - Dispatch
 * c) async - Wait for response
 * d) sync - Serialize
 *
 */

impl<T, F, S> Future for ResponseFuture<F, S>
where
    T: Response,
    F: Future<Item = T>,
{
    type Item = http::Response<T::Body>;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        unimplemented!();
    }
}
