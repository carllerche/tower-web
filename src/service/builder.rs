use response::DefaultSerializer;
use service::{Resource, IntoResource, WebService};
use util::{BufStream, Chain};

use std::io;
use std::marker::PhantomData;
use std::net::SocketAddr;

/// Builds a web service
#[derive(Debug)]
pub struct ServiceBuilder<T, RequestBody> {
    /// The inner resource
    resource: T,
    _p: PhantomData<RequestBody>,
}

impl<RequestBody> ServiceBuilder<(), RequestBody> {
    /// Create a new `ServiceBuilder`
    pub fn new() -> Self {
        ServiceBuilder {
            resource: (),
            _p: PhantomData,
        }
    }
}

impl<T, RequestBody> ServiceBuilder<T, RequestBody> {
    /// Add a resource handler.
    pub fn resource<U>(self, resource: U) -> ServiceBuilder<<T as Chain<U>>::Output, RequestBody>
    where
        T: Chain<U>,
    {
        ServiceBuilder {
            resource: self.resource.chain(resource),
            _p: PhantomData,
        }
    }
}

impl<T, RequestBody> ServiceBuilder<T, RequestBody>
where
    T: IntoResource<DefaultSerializer, RequestBody>,
    RequestBody: BufStream,
{
    /// Build a service instance.
    pub fn build(self) -> WebService<T::Resource, RequestBody> {
        let routes = self.resource.routes();
        WebService::new(self.resource.into_resource(DefaultSerializer::new()), routes)
    }
}

impl<T> ServiceBuilder<T, ::run::LiftReqBody>
where
    T: IntoResource<DefaultSerializer, ::run::LiftReqBody>,
    T::Resource: Send + 'static,
    <T::Resource as Resource>::Buf: Send,
    <T::Resource as Resource>::Body: Send,
    <T::Resource as Resource>::Future: Send,
{
    /// Run the service
    pub fn run(self, addr: &SocketAddr) -> io::Result<()> {
        fn assert_send<T: Send>() {}
        assert_send::<::run::LiftReqBody>();
        assert_send::<DefaultSerializer>();
        ::run::run(addr, self.build())
    }
}
