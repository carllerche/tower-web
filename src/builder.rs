use {Resource, Service, Payload};
use resource::Chain;

use std::io;
use std::net::SocketAddr;

/// Builds a web service
#[derive(Debug)]
pub struct ServiceBuilder<T> {
    /// The inner resource
    resource: T,
}

impl ServiceBuilder<()> {
    /// Create a new `ServiceBuilder`
    pub fn new() -> Self {
        ServiceBuilder { resource: () }
    }
}

impl<T> ServiceBuilder<T>
where
    T: Resource,
{
    /// Add a resource handler.
    pub fn resource<U>(self, resource: U) -> ServiceBuilder<<T as Chain<U>>::Resource>
    where
        U: Resource,
        T: Chain<U>,
    {
        ServiceBuilder {
            resource: self.resource.chain(resource),
        }
    }

    /// Build a service instance.
    pub fn build<P: Payload>(self) -> Service<T, P> {
        Service::new(self.resource)
    }
}

impl<T> ServiceBuilder<T>
where
    T: Resource,
    T::Buf: Send,
{
    /// Run the service
    pub fn run(self, addr: &SocketAddr) -> io::Result<()> {
        ::run::run(addr, self.build())
    }
}
