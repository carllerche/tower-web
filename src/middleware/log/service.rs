use futures::{Future, Poll};
use http;
use tower_service::Service;

use std::time::Instant;

pub struct LogService<S> {
    inner: S,
    target: &'static str,
}

pub struct ResponseFuture<T> {
    inner: T,
    method: http::Method,
    path: Option<http::uri::PathAndQuery>,
    version: http::Version,
    start: Instant,
    target: &'static str,
}

impl<S> LogService<S> {
    pub(super) fn new(inner: S, target: &'static str) -> LogService<S> {
        LogService {
            inner,
            target,
        }
    }
}

impl<S, RequestBody, ResponseBody> Service for LogService<S>
where S: Service<Request = http::Request<RequestBody>,
                Response = http::Response<ResponseBody>>,
      S::Error: ::std::error::Error,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_ready()
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        let method = request.method().clone();
        let path = request.uri().path_and_query().map(|p| p.clone());
        let version = request.version();
        let start = Instant::now();

        let inner = self.inner.call(request);

        ResponseFuture {
            inner,
            method,
            path,
            version,
            start,
            target: self.target,
        }
    }
}

impl<T, B> Future for ResponseFuture<T>
where
    T: Future<Item = http::Response<B>>,
    T::Error: ::std::error::Error,
{
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use futures::Async::*;

        let res = self.inner.poll();

        match res {
            Ok(Ready(ref response)) => {
                let full_path = self.path.as_ref()
                    .map(|p| p.as_str())
                    .unwrap_or("/");

                // TODO:
                // - remote_addr
                // - response content length
                // - date
                info!(
                    target: self.target,
                    "\"{} {} {:?}\" {} {:?}",
                    self.method,
                    full_path,
                    self.version,
                    response.status().as_u16(),
                    self.start.elapsed(),
                );
            }
            Err(ref err) => {
                warn!("ERROR: {}", err);
            }
            _ => {}
        }

        res
    }
}
