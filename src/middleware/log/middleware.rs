use super::LogService;
use middleware::Middleware;

use http;
use tower_service::Service;

#[derive(Debug)]
pub struct LogMiddleware {
    target: &'static str,
}

impl LogMiddleware {
    pub fn new(target: &'static str) -> LogMiddleware {
        LogMiddleware { target }
    }
}

impl<S, RequestBody, ResponseBody> Middleware<S> for LogMiddleware
where S: Service<Request = http::Request<RequestBody>,
                Response = http::Response<ResponseBody>>,
      S::Error: ::std::error::Error,
{
    type Request = http::Request<RequestBody>;
    type Response = http::Response<ResponseBody>;
    type Error = S::Error;
    type Service = LogService<S>;

    fn wrap(&self, service: S) -> Self::Service {
        LogService::new(service, self.target)
    }
}
