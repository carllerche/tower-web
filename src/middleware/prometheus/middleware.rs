use super::PrometheusService;
use middleware::Middleware;

use http;
use prometheus::{CounterVec, HistogramVec, Opts, Registry};
use tower_service::Service;

/// Instrument HTTP calls.
pub struct PrometheusMiddleware {
    /// Support prefixing metrics with a namespace.
    counter_vec: CounterVec,
    histogram_vec: HistogramVec,
    // TODO: Support switching between histograms and summaries.
}

impl PrometheusMiddleware {
    /// Create a new `PrometheusMiddleware` instance.
    pub fn new(namespace: &'static str, registry: Registry) -> PrometheusMiddleware {
        let counter_vec_opts =
            Opts::new("test_counter_vec_d", "test counter vector help").namespace(namespace);
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

        PrometheusMiddleware {
            counter_vec,
            histogram_vec,
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
            self.counter_vec.clone(),
            self.histogram_vec.clone(),
        )
    }
}
