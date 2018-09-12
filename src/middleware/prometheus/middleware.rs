use super::PrometheusService;
use middleware::Middleware;

use http;
use prometheus::Registry;
use tower_service::Service;

/// Instrument HTTP calls.
pub struct PrometheusMiddleware {
    /// Support prefixing metrics with a namespace.
    namespace: &'static str,
    registry: Registry,
    // TODO: Support switching between histograms and summaries.
}

impl PrometheusMiddleware {
    /// Create a new `PrometheusMiddleware` instance.
    pub fn new(namespace: &'static str, registry: Registry) -> PrometheusMiddleware {
        PrometheusMiddleware {
            namespace,
            registry,
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
        PrometheusService::new(service, self.namespace, &self.registry)
    }
}
