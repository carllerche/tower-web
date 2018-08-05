use super::{Config, CorsService};
use middleware::Middleware;
use util::buf_stream::BufStream;

use http;
use tower_service::Service;

use std::sync::Arc;

#[derive(Debug)]
pub struct CorsMiddleware {
    config: Arc<Config>,
}

impl CorsMiddleware {
    pub(super) fn new(config: Config) -> CorsMiddleware {
        let config = Arc::new(config);
        CorsMiddleware { config }
    }
}

impl<S, RequestBody, ResponseBody> Middleware<S> for CorsMiddleware
where
    S: Service<Request = http::Request<RequestBody>, Response = http::Response<ResponseBody>>,
    ResponseBody: BufStream,
{
    type Request = http::Request<RequestBody>;
    type Response = http::Response<Option<ResponseBody>>;
    type Error = S::Error;
    type Service = CorsService<S>;

    fn wrap(&self, service: S) -> Self::Service {
        CorsService::new(service, self.config.clone())
    }
}
