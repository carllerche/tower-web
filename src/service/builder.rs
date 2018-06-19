use service::{WebService, Payload, Resource};
use response::DefaultSerializer;
use util::Chain;

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
    pub fn resource<U>(self, resource: U) -> ServiceBuilder<<T as Chain<U>>::Output>
    where
        U: Resource,
        T: Chain<U>,
    {
        ServiceBuilder {
            resource: self.resource.chain(resource),
        }
    }

    /// Build a service instance.
    pub fn build<P: Payload>(self) -> WebService<T, DefaultSerializer, P> {
        WebService::new(self.resource, DefaultSerializer::new())
    }
}

impl<T> ServiceBuilder<T>
where
    T: Resource,
    T::Buf: Send,
    T::Body: Send,
    T::Future: Send,
{
    /// Run the service
    pub fn run(self, addr: &SocketAddr) -> io::Result<()> {
        ::run::run(addr, self.build())
    }
}
