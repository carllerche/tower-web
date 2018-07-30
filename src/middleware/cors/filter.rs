use super::{CorsService, Config};
use middleware::Filter;

use http;
use tower_service::Service;

use std::sync::Arc;

#[derive(Debug)]
pub struct CorsFilter {
    config: Arc<Config>,
}

impl CorsFilter {
    pub(super) fn new(config: Config) -> CorsFilter {
        let config = Arc::new(config);
        CorsFilter { config }
    }
}

impl<S, RequestBody, ResponseBody> Filter<S> for CorsFilter
where S: Service<Request = http::Request<RequestBody>,
                Response = http::Response<ResponseBody>>,
{
    type Service = CorsService<S>;

    fn wrap(&self, service: S) -> Self::Service {
        CorsService::new(service, self.config.clone())
    }
}
