use middleware::Identity;
use response::DefaultSerializer;
use service::{Resource, IntoResource, WebService};
use util::{BufStream, Chain};

use std::io;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::Arc;

/// Builds a web service
///
/// TODO: Is `RequestBody` required here?
#[derive(Debug)]
pub struct ServiceBuilder<T, Middleware, RequestBody> {
    /// The inner resource
    resource: T,
    middleware: Middleware,
    _p: PhantomData<RequestBody>,
}

impl<RequestBody> ServiceBuilder<(), Identity, RequestBody> {
    /// Create a new `ServiceBuilder`
    pub fn new() -> Self {
        ServiceBuilder {
            resource: (),
            middleware: Identity::new(),
            _p: PhantomData,
        }
    }
}

impl<T, Middleware, RequestBody> ServiceBuilder<T, Middleware, RequestBody> {
    /// Add a resource handler.
    pub fn resource<U>(self, resource: U)
        -> ServiceBuilder<<T as Chain<U>>::Output, Middleware, RequestBody>
    where
        T: Chain<U>,
    {
        ServiceBuilder {
            resource: self.resource.chain(resource),
            middleware: self.middleware,
            _p: PhantomData,
        }
    }
}

impl<T, Middleware, RequestBody> ServiceBuilder<T, Middleware, RequestBody>
where
    T: IntoResource<DefaultSerializer, RequestBody>,
    // Middleware: Middleware<?>,
    RequestBody: BufStream,
{
    /// Build a service instance.
    pub fn build(self) -> WebService<T::Resource, RequestBody> {
        let routes = Arc::new(self.resource.routes());

        WebService::new(
            self.resource.into_resource(DefaultSerializer::new()),
            routes)
    }
}

impl<T, M> ServiceBuilder<T, M, ::run::LiftReqBody>
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
