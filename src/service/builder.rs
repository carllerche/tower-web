use middleware::Identity;
use response::DefaultSerializer;
use service::{Resource, IntoResource, WebService};
use util::{BufStream, Chain};

use std::io;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::Arc;

/// Builds a web service
#[derive(Debug)]
pub struct ServiceBuilder<T, Middleware> {
    /// The inner resource
    resource: T,
    middleware: Middleware,
}

impl ServiceBuilder<(), Identity> {
    /// Create a new `ServiceBuilder`
    pub fn new() -> Self {
        ServiceBuilder {
            resource: (),
            middleware: Identity::new(),
        }
    }
}

impl<T, Middleware> ServiceBuilder<T, Middleware> {
    /// Add a resource handler.
    pub fn resource<U>(self, resource: U)
        -> ServiceBuilder<<T as Chain<U>>::Output, Middleware>
    where
        T: Chain<U>,
    {
        ServiceBuilder {
            resource: self.resource.chain(resource),
            middleware: self.middleware,
        }
    }

    /// Build a service instance.
    pub fn build<RequestBody>(self) -> WebService<T::Resource, RequestBody>
    where T: IntoResource<DefaultSerializer, RequestBody>,
          RequestBody: BufStream,
    {
        let routes = Arc::new(self.resource.routes());

        WebService::new(
            self.resource.into_resource(DefaultSerializer::new()),
            routes)
    }

    /// Run the service
    pub fn run(self, addr: &SocketAddr) -> io::Result<()>
    where T: IntoResource<DefaultSerializer, ::run::LiftReqBody>,
          T::Resource: Send + 'static,
          <T::Resource as Resource>::Buf: Send,
          <T::Resource as Resource>::Body: Send,
          <T::Resource as Resource>::Future: Send,
    {
        fn assert_send<T: Send>() {}
        assert_send::<::run::LiftReqBody>();
        assert_send::<DefaultSerializer>();
        ::run::run(addr, self.build())
    }
}
