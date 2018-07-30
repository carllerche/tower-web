use super::Config;

use futures::{Poll};
use http;
use tower_service::Service;

use std::sync::Arc;

pub struct CorsService<S> {
    inner: S,
    config: Arc<Config>,
}

impl<S> CorsService<S> {
    pub(super) fn new(inner: S, config: Arc<Config>) -> CorsService<S> {
        CorsService {
            inner,
            config,
        }
    }
}

impl<S, RequestBody, ResponseBody> Service for CorsService<S>
where S: Service<Request = http::Request<RequestBody>,
                Response = http::Response<ResponseBody>>,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_ready()
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        self.inner.call(request)
    }
}
