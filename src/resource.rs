use futures::{Future};
use futures::future::FutureResult;
use http;

/// A resource
pub trait Resource: Clone + Send + 'static {
    type Future: Future<Item = http::Response<String>> + Send + 'static;

    fn call(&mut self) -> Self::Future;
}

/// Resource that matches all requests and returns 404.
#[derive(Clone)]
pub struct NotFound(());

impl NotFound {
    pub fn new() -> NotFound {
        NotFound(())
    }
}

impl Resource for NotFound {
    type Future = FutureResult<http::Response<String>, ()>;

    fn call(&mut self) -> Self::Future {
        unimplemented!();
    }
}
