use error::{IntoCatch, DefaultCatch};
use middleware::Identity;
use response::DefaultSerializer;
use routing::{Resource, IntoResource, RoutedService};
use service::NewWebService;
use util::{BufStream, Chain};
use util::http::{HttpService, HttpMiddleware};

use std::io;
use std::net::SocketAddr;

/// Builds a web service
#[derive(Debug)]
pub struct ServiceBuilder<T, C, Middleware> {
    /// The inner resource
    resource: T,
    catch: C,
    middleware: Middleware,
}

impl ServiceBuilder<(), DefaultCatch, Identity> {
    /// Create a new `ServiceBuilder`
    pub fn new() -> Self {
        ServiceBuilder {
            resource: (),
            catch: DefaultCatch::new(),
            middleware: Identity::new(),
        }
    }
}

impl<T, C, M> ServiceBuilder<T, C, M> {
    /// Add a resource handler.
    pub fn resource<U>(self, resource: U)
        -> ServiceBuilder<<T as Chain<U>>::Output, C, M>
    where
        T: Chain<U>,
    {
        ServiceBuilder {
            resource: self.resource.chain(resource),
            catch: self.catch,
            middleware: self.middleware,
        }
    }

    /// Add a middleware.
    pub fn middleware<U>(self, middleware: U)
        -> ServiceBuilder<T, C, <M as Chain<U>>::Output>
    where
        M: Chain<U>,
    {
        ServiceBuilder {
            resource: self.resource,
            catch: self.catch,
            middleware: self.middleware.chain(middleware),
        }
    }

    pub fn catch<U>(self, catch: U) -> ServiceBuilder<T, U, M> {
        ServiceBuilder {
            resource: self.resource,
            catch,
            middleware: self.middleware,
        }
    }

    /// Build a `NewWebService` instance
    ///
    /// This instance is used to generate service values.
    pub fn build_new_service<RequestBody>(self) -> NewWebService<T::Resource, C::Catch, M>
    where T: IntoResource<DefaultSerializer, RequestBody>,
          C: IntoCatch<DefaultSerializer>,
          M: HttpMiddleware<RoutedService<T::Resource, C::Catch>>,
          RequestBody: BufStream,
    {
        // Build the routes
        let routes = self.resource.routes();
        let serializer = DefaultSerializer::new();

        // Create the routed service
        let routed = RoutedService::new(
            self.resource.into_resource(serializer),
            self.catch.into_catch(),
            routes);

        NewWebService::new(
            routed,
            self.middleware)
    }

    /// Run the service
    pub fn run(self, addr: &SocketAddr) -> io::Result<()>
    where T: IntoResource<DefaultSerializer, ::run::LiftReqBody>,
          C: IntoCatch<DefaultSerializer> + Send + 'static,
          C::Catch: Send,
          M: HttpMiddleware<RoutedService<T::Resource, C::Catch>, RequestBody = ::run::LiftReqBody> + Send + 'static,
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
