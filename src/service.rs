use Resource;
use routing::RouteSet;

use http;
use tower;
use tokio::prelude::*;

use std::sync::Arc;

/// Web service
#[derive(Clone, Debug)]
pub struct Service<T> {
    resource: T,
    routes: Arc<RouteSet>,
}

impl<T> Service<T>
where T: Resource,
{
    pub(crate) fn new(resource: T) -> Self {
        let routes = Arc::new(resource.routes());

        Service {
            resource,
            routes,
        }
    }
}

impl<T> tower::Service for Service<T>
where T: Resource,
{
    type Request = http::Request<String>;
    type Response = http::Response<T::Body>;
    type Error = ::Error;
    type Future = T::Future;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        // Always ready
        Ok(().into())
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        // TODO: Use the body
        let (head, _) = request.into_parts();
        let request = http::Request::from_parts(head, ());

        match self.routes.test(&request) {
            Some(match_) => {
                self.resource.dispatch(&match_, request)
            }
            None => {
                unimplemented!();
                /*
                let response = ::tower_web::codegen::http::Response::builder()
                    .status(404)
                    .header("content-type", "text/plain")
                    .body(Box::new(body) as Self::Body)
                    .unwrap();

                Box::new(future::ok(response))
                */
            }
        }
    }
}
