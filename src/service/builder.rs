use middleware::{Identity};
use response::DefaultSerializer;
use routing::{Resource, IntoResource};
use service::{WebService, HttpService, NewWebService, HttpMiddleware};
use util::{BufStream, Chain};

use std::io;
use std::net::SocketAddr;

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

impl<T, M> ServiceBuilder<T, M> {
    /// Add a resource handler.
    pub fn resource<U>(self, resource: U)
        -> ServiceBuilder<<T as Chain<U>>::Output, M>
    where
        T: Chain<U>,
    {
        ServiceBuilder {
            resource: self.resource.chain(resource),
            middleware: self.middleware,
        }
    }

    /// Add a middleware.
    pub fn middleware<U>(self, middleware: U)
        -> ServiceBuilder<T, <M as Chain<U>>::Output>
    where
        M: Chain<U>,
    {
        ServiceBuilder {
            resource: self.resource,
            middleware: self.middleware.chain(middleware),
        }
    }

    /// Build a `NewWebService` instance
    ///
    /// This instance is used to generate service values.
    pub fn build_new_service<RequestBody>(self) -> NewWebService<T::Resource, M>
    where T: IntoResource<DefaultSerializer, RequestBody>,
          M: HttpMiddleware<WebService<T::Resource>>,
          RequestBody: BufStream,
    {
        // Build the routes
        let routes = self.resource.routes();
        let serializer = DefaultSerializer::new();

        NewWebService::new(
            self.resource.into_resource(serializer),
            self.middleware,
            routes)
    }

    /// Run the service
    pub fn run(self, addr: &SocketAddr) -> io::Result<()>
    where T: IntoResource<DefaultSerializer, ::run::LiftReqBody>,
          M: HttpMiddleware<WebService<T::Resource>, RequestBody = ::run::LiftReqBody> + Send + 'static,
          M::Service: Send,
          <M::Service as HttpService>::Future: Send,
          M::ResponseBody: Send,
          <M::ResponseBody as BufStream>::Item: Send,
          T::Resource: Send + 'static,
          <T::Resource as Resource>::Buf: Send,
          <T::Resource as Resource>::Body: Send,
          <T::Resource as Resource>::Future: Send,
    {
        ::run::run(addr, self.build_new_service())
    }
}
