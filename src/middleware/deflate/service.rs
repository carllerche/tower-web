use flate2::Compression;
use futures::{Future, Poll};
use http;
use http::header::{CONTENT_ENCODING, HeaderValue};
use tower_service::Service;
use crate::util::buf_stream::BufStream;
use crate::util::buf_stream::deflate::CompressStream;

/// Deflates the inner service's response bodies.
#[derive(Debug)]
pub struct DeflateService<S> {
    inner: S,
    level: Compression,
}

/// Deflate the response body.
#[derive(Debug)]
pub struct ResponseFuture<T> {
    inner: T,
    level: Compression,
}

impl<S> DeflateService<S> {
    pub(super) fn new(inner: S, level: Compression) -> DeflateService<S> {
        DeflateService {
            inner,
            level,
        }
    }
}

impl<S, RequestBody, ResponseBody> Service for DeflateService<S>
where S: Service<Request = http::Request<RequestBody>,
                Response = http::Response<ResponseBody>>,
      ResponseBody: BufStream,
      S::Error: ::std::error::Error,
{
    type Request = http::Request<RequestBody>;
    type Response = http::Response<CompressStream<ResponseBody>>;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_ready()
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        ResponseFuture {
            inner: self.inner.call(request),
            level: self.level,
        }
    }
}

impl<T, B> Future for ResponseFuture<T>
where
    T: Future<Item = http::Response<B>>,
    B: BufStream,
    T::Error: ::std::error::Error,
{
    type Item = http::Response<CompressStream<B>>;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut response = try_ready!(self.inner.poll())
            .map(|body| CompressStream::new(body, self.level));

        let content_encoding = HeaderValue::from_static("deflate");

        // Set content-encoding
        response.headers_mut()
            .insert(CONTENT_ENCODING, content_encoding);

        Ok(response.into())
    }
}
