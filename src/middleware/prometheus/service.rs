use futures::{Future, Poll};
use http;
use prometheus::{CounterVec, Histogram, HistogramVec};
use std::time::Instant;
use tower_service::Service;

/// Decorates a service by instrumenting all received requests
pub struct PrometheusService<S> {
    inner: S,
    counter_vec: CounterVec,
    histogram_vec: HistogramVec,
}

/// TODO: fix derive(debug)
pub struct ResponseFuture<T> {
    inner: T,
    method: http::Method,
    path: http::uri::PathAndQuery,
    start: Instant,
    histogram: Histogram,
    counter_vec: CounterVec,
}

impl<S> PrometheusService<S> {
    pub(super) fn new(
        inner: S,
        counter_vec: CounterVec,
        histogram_vec: HistogramVec,
    ) -> PrometheusService<S> {
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
        let path = request.uri().path_and_query().map(|p| p.clone()).unwrap();

        let histogram = self
            .histogram_vec
            .with_label_values(&[path.path(), method.as_str()]);

        let inner = self.inner.call(request);
        let start = Instant::now();
        let counter_vec = self.counter_vec.clone();

        let size = request.body().length();

        ResponseFuture {
            inner,
            method,
            path,
            start,
            histogram,
            counter_vec,
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
                let elapsed = self.start.elapsed();
                let nanos = f64::from(elapsed.subsec_nanos()) / 1e9;
                self.histogram.observe(elapsed.as_secs() as f64 + nanos);

                self.counter_vec
                    .with_label_values(&[
                        self.path.path(),
                        response.status().as_str(),
                        self.method.as_str(),
                    ]).inc();
            }
            Err(ref err) => {
                warn!("ERROR: {}", err);
            }
            _ => {}
        }

        res
    }
}
