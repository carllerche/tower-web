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

use util::http::HttpService;
use http::{Request, Response};

impl<S> Service for CorsService<S>
where S: HttpService,
{
    type Request = Request<S::RequestBody>;
    type Response = Response<S::ResponseBody>;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_ready()
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        self.inner.call(request)
    }
}
