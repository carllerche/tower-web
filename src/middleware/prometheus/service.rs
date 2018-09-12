use futures::{Future, Poll};
use http;
use prometheus::{CounterVec, Encoder, HistogramVec, Opts, Registry, TextEncoder};
use tower_service::Service;

use std::time::Instant;

/// Decorates a service by instrumenting all received requests
pub struct PrometheusService<S> {
    inner: S,
    counter_vec: CounterVec,
    histogram_vec: HistogramVec,
}

/// TODO
#[derive(Debug)]
pub struct ResponseFuture<T> {
    inner: T,
    method: http::Method,
    path: Option<http::uri::PathAndQuery>,
    version: http::Version,
    start: Instant,
}

impl<S> PrometheusService<S> {
    pub(super) fn new(
        inner: S,
        namespace: &'static str,
        registry: &Registry,
    ) -> PrometheusService<S> {
        let counter_vec_opts =
            Opts::new("test_counter_vec", "test counter vector help").namespace(namespace);
        let counter_vec =
            CounterVec::new(counter_vec_opts, &["path", "statusCode", "method"]).unwrap();

        // TODO: Note that histogram doesn't report failures.
        let histogram_opts = histogram_opts!(
            "test_histogram_opts",
            "test histogram help",
            // TODO: Decide on buckets. Expose them to users?
            vec![0.1, 0.2, 0.5, 1.0, 2.0, 5.0]
        ).namespace(namespace);

        let histogram_vec = HistogramVec::new(histogram_opts, &["path", "method"]).unwrap();

        registry.register(Box::new(counter_vec.clone())).unwrap();
        registry.register(Box::new(histogram_vec.clone())).unwrap();

        PrometheusService {
            inner,
            counter_vec,
            histogram_vec,
        }
    }
}

impl<S, RequestBody, ResponseBody> Service for PrometheusService<S>
where
    S: Service<Request = http::Request<RequestBody>, Response = http::Response<ResponseBody>>,
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
                let full_path = self.path.as_ref().map(|p| p.as_str()).unwrap_or("/");

                let elapsed = self.start;

                // TODO: Instrument response

                // info!(
                //     target: self.target,
                //     "\"{} {} {:?}\" {} {:?}",
                //     self.method,
                //     full_path,
                //     self.version,
                //     response.status().as_u16(),
                //     self.start.elapsed(),
                // );
            }
            Err(ref err) => {
                warn!("ERROR: {}", err);
            }
            _ => {}
        }

        res
    }
}
