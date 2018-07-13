use response::DefaultSerializer;
use service::{Resource, IntoResource, WebService};
use util::{BufStream, Chain};

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
    T: IntoResource<DefaultSerializer>,
{
    /// Add a resource handler.
    pub fn resource<U>(self, resource: U) -> ServiceBuilder<<T as Chain<U>>::Output>
    where
        U: IntoResource<DefaultSerializer>,
        T: Chain<U>,
    {
        ServiceBuilder {
            resource: self.resource.chain(resource),
        }
    }

    /// Build a service instance.
    pub fn build<In: BufStream>(self) -> WebService<T::Resource, In> {
        WebService::new(self.resource.into_resource(DefaultSerializer::new()))
    }
}

impl<T> ServiceBuilder<T>
where
    T: IntoResource<DefaultSerializer>,
    T::Resource: Send + 'static,
    <T::Resource as Resource>::Buf: Send,
    <T::Resource as Resource>::Body: Send,
    <T::Resource as Resource>::Future: Send,
{
    /// Run the service
    pub fn run(self, addr: &SocketAddr) -> io::Result<()> {
        ::run::run(addr, self.build())
    }
}
