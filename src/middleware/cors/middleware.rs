use super::{Config, CorsService};
use crate::middleware::Middleware;

use http;
use crate::util::http::HttpService;

use std::sync::Arc;

/// Middleware providing an implementation of the CORS specification.
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

impl<S> Middleware<S> for CorsMiddleware
where
    S: HttpService,
{
    type Request = http::Request<S::RequestBody>;
    type Response = http::Response<Option<S::ResponseBody>>;
    type Error = S::Error;
    type Service = CorsService<S>;

    fn wrap(&self, service: S) -> Self::Service {
        CorsService::new(service, self.config.clone())
    }
}
