use Resource;

use http;
use tower;
use tokio::prelude::*;

/// Web service
#[derive(Clone, Debug)]
pub struct Service<T> {
    resource: T,
}

impl<T> Service<T>
where T: Resource,
{
    pub(crate) fn new(resource: T) -> Self {
        Service {
            resource,
        }
    }
}

impl<T> tower::Service for Service<T>
where T: Resource,
{
    type Request = http::Request<String>;
    type Response = http::Response<T::Body>;
    type Error = T::Error;
    type Future = T::Future;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        // Always ready
        Ok(().into())
    }

    fn call(&mut self, _request: Self::Request) -> Self::Future {
        // TODO: Do something with the request o_O

        self.resource.call()
    }
}
