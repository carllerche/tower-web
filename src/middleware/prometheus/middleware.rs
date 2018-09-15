use super::PrometheusService;
use middleware::Middleware;

use http;
use prometheus::{CounterVec, HistogramVec, Opts, Registry};
use tower_service::Service;

/// Instrument HTTP calls.
pub struct PrometheusMiddleware {
    requests_total_counter_vec: CounterVec,
    request_bytes_counter_vec: CounterVec,
    response_bytes_counter_vec: CounterVec,
    request_duration_histogram_vec: HistogramVec,
    // TODO: Support switching between histograms and summaries.
}

impl PrometheusMiddleware {
    /// Create a new `PrometheusMiddleware` instance.
    pub fn new(namespace: Option<&'static str>, registry: Registry) -> PrometheusMiddleware {
        let requests_total_counter_opts =
            Opts::new("http_requests_total", "Counter of HTTP requests.")
                .namespace(namespace.unwrap_or(""));
        let requests_total_counter_vec = CounterVec::new(
            requests_total_counter_opts,
            &["path", "statusCode", "method"],
        ).unwrap();

        let request_bytes_counter_opts = Opts::new(
            "http_request_bytes_total",
            "Counter of HTTP request body bytes.",
        ).namespace(namespace.unwrap_or(""));
        let request_bytes_counter_vec = CounterVec::new(
            request_bytes_counter_opts,
            &["path", "statusCode", "method"],
        ).unwrap();

        let response_bytes_counter_opts = Opts::new(
            "http_response_bytes_total",
            "Counter of HTTP response body bytes.",
        ).namespace(namespace.unwrap_or(""));
        let response_bytes_counter_vec = CounterVec::new(
            response_bytes_counter_opts,
            &["path", "statusCode", "method"],
        ).unwrap();

        // TODO: Note that histogram doesn't report failures.
        let histogram_opts = histogram_opts!(
            "http_request_duration_seconds",
            "Histogram of HTTP request duration in seconds.",
            // TODO: Decide on buckets. Expose them to users?
            vec![0.1, 0.2, 0.5, 1.0, 2.0, 5.0]
        ).namespace(namespace.unwrap_or(""));
        let request_duration_histogram_vec =
            HistogramVec::new(histogram_opts, &["path", "method"]).unwrap();

        registry
            .register(Box::new(requests_total_counter_vec.clone()))
            .unwrap();
        registry
            .register(Box::new(request_bytes_counter_vec.clone()))
            .unwrap();
        registry
            .register(Box::new(response_bytes_counter_vec.clone()))
            .unwrap();
        registry
            .register(Box::new(request_duration_histogram_vec.clone()))
            .unwrap();

        PrometheusMiddleware {
            requests_total_counter_vec,
            request_bytes_counter_vec,
            response_bytes_counter_vec,
            request_duration_histogram_vec,
        }
    }
}

impl<S, RequestBody, ResponseBody> Middleware<S> for PrometheusMiddleware
where
    S: Service<Request = http::Request<RequestBody>, Response = http::Response<ResponseBody>>,
    S::Error: ::std::error::Error,
{
    type Request = http::Request<RequestBody>;
    type Response = http::Response<ResponseBody>;
    type Error = S::Error;
    type Service = PrometheusService<S>;

    fn wrap(&self, service: S) -> Self::Service {
        PrometheusService::new(
            service,
            self.requests_total_counter_vec.clone(),
            self.request_bytes_counter_vec.clone(),
            self.response_bytes_counter_vec.clone(),
            self.request_duration_histogram_vec.clone(),
        )
    }
}
