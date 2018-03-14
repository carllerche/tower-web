use {NotFound, Resource, Service};

use std::io;
use std::net::SocketAddr;

/// Builds a web service
#[derive(Debug)]
pub struct ServiceBuilder<T> {
    /// The inner resource
    resource: T,
}

impl ServiceBuilder<NotFound> {
    /// Create a new `ServiceBuilder`
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
    pub fn build(self) -> Service<T> {
        Service::new(self.resource)
    }

    /// Run the service
    pub fn run(self, addr: &SocketAddr) -> io::Result<()> {
        ::run::run(addr, self.build())
    }
}
