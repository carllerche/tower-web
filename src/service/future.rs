use response::{Context, Serializer, Response};

use futures::{Future, Poll};
use http;

// ===== HttpResponseFuture =====

pub trait HttpResponseFuture {
    type Item;

    fn poll_http_response(&mut self) -> Poll<http::Response<Self::Item>, ::Error>;
}

impl<T, B> HttpResponseFuture for T
where T: Future<Item = http::Response<B>, Error = ::Error>
{
    type Item = B;

    fn poll_http_response(&mut self) -> Poll<http::Response<Self::Item>, ::Error> {
        self.poll()
    }
}
