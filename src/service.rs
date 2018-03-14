use {Resource, NotFound};

use http;
use tower;
use tokio::prelude::*;

use std::io;
use std::net::SocketAddr;

/// Builds a web service
#[derive(Debug)]
pub struct ServiceBuilder<T> {
    /// The inner resource
    resource: T,
}

#[derive(Clone, Debug)]
pub struct Service<T> {
    resource: T,
}

// ===== impl ServiceBuilder =====

impl ServiceBuilder<NotFound> {
    pub fn new() -> Self {
        ServiceBuilder {
            resource: NotFound::new(),
        }
    }
}

impl<T> ServiceBuilder<T>
where T: Resource,
{
    /// Add a resource handler.
    pub fn resource<U>(self, resource: U) -> ServiceBuilder<U>
    where U: Resource,
    {
        ServiceBuilder {
            resource: resource,
        }
    }

    /// Build a service instance.
    pub fn build(self) {
        unimplemented!();
    }

    /// Run the service
    pub fn run(self, addr: &SocketAddr) -> io::Result<()> {
        ::run::run(addr, Service {
            resource: self.resource,
        })
    }
}

// ===== impl Service =====

impl<T> tower::Service for Service<T>
where T: Resource,
{
    type Request = http::Request<String>;
    type Response = http::Response<String>;
    type Error = <T::Future as Future>::Error;
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
