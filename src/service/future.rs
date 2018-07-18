use response::{Context, Serializer, Response};

use futures::{Future, Poll};
use http;

// ===== IntoHttpFuture =====

// TODO: Rename `response::ResponseFuture`?
pub trait IntoHttpFuture {
    type Item;

    fn poll_into_http<S: Serializer>(&mut self, context: &Context<S>)
        -> Poll<http::Response<Self::Item>, ::Error>;
}

impl<T, R> IntoHttpFuture for T
where T: Future<Item = R, Error = ::Error>,
      R: Response
{
    type Item = R::Body;

    fn poll_into_http<S: Serializer>(&mut self, context: &Context<S>)
        -> Poll<http::Response<Self::Item>, ::Error>
    {
        let response = try_ready!(self.poll());
        Ok(response.into_http(context).into())
    }
}

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
