use futures::{Future, Poll};
use http;
use prometheus::{CounterVec, Histogram, HistogramVec};
use std::time::Instant;
use tower_service::Service;

/// Decorates a service by instrumenting all received requests
pub struct PrometheusService<S> {
    inner: S,
    requests_total_counter_vec: CounterVec,
    request_bytes_counter_vec: CounterVec,
    response_bytes_counter_vec: CounterVec,
    request_duration_histogram_vec: HistogramVec,
}

/// TODO: fix derive(debug)
pub struct ResponseFuture<T> {
    inner: T,
    method: http::Method,
    path: http::uri::PathAndQuery,
    start: Instant,
    request_duration_histogram: Histogram,
    requests_total_counter_vec: CounterVec,
}

impl<S> PrometheusService<S> {
    pub(super) fn new(
        inner: S,
        requests_total_counter_vec: CounterVec,
        request_bytes_counter_vec: CounterVec,
        response_bytes_counter_vec: CounterVec,
        request_duration_histogram_vec: HistogramVec,
    ) -> PrometheusService<S> {
        PrometheusService {
            inner,
            requests_total_counter_vec,
            request_bytes_counter_vec,
            response_bytes_counter_vec,
            request_duration_histogram_vec,
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

        let request_duration_histogram = self
            .request_duration_histogram_vec
            .with_label_values(&[path.path(), method.as_str()]);

        let inner = self.inner.call(request);
        let start = Instant::now();
        let requests_total_counter_vec = self.requests_total_counter_vec.clone();

        // TODO: How do I get the length of the request body?

        ResponseFuture {
            inner,
            method,
            path,
            start,
            request_duration_histogram,
            requests_total_counter_vec,
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
                self.request_duration_histogram.observe(elapsed.as_secs() as f64 + nanos);

                // TODO: How do I get the length of the response body?

                self.requests_total_counter_vec
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
