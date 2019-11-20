use crate::config::ConfigBuilder;
use crate::error::{IntoCatch, DefaultCatch};
use futures::Future;
use crate::middleware::Identity;
use crate::net::ConnectionStream;
use crate::response::{DefaultSerializer, Serializer};
use crate::routing::{Resource, IntoResource, RoutedService};
use crate::service::NewWebService;
use crate::util::{BufStream, Chain};
use crate::util::http::{HttpService, HttpMiddleware};

use std::io;
use std::net::SocketAddr;

/// Configure and build a web service.
///
/// `ServiceBuilder` collects all the components and configuration required to
/// build the HTTP service. Once the service is defined, it can be run
/// with `run`.
///
/// # Examples
///
/// Defining a service with a single resource;
///
/// ```rust
/// # #[macro_use] extern crate tower_web;
/// use tower_web::ServiceBuilder;
///
/// struct MyResource;
///
/// impl_web! {
///     impl MyResource {
///         // ...
///     }
/// }
///
/// # fn main() {
/// # if false {
/// # let addr = "127.0.0.1:8080".parse().unwrap();
/// ServiceBuilder::new()
///     .resource(MyResource)
///     .run(&addr);
/// # }
/// # }
/// ```
///
/// Defining a service with a multiple resources;
///
/// ```rust
/// # #[macro_use] extern crate tower_web;
/// use tower_web::ServiceBuilder;
///
/// struct MyResource1;
/// struct MyResource2;
/// struct MyResource3;
///
/// impl_web! {
///     impl MyResource1 {
///         // ...
///     }
///
///     impl MyResource2 {
///         // ...
///     }
///
///     impl MyResource3 {
///         // ...
///     }
/// }
///
/// # fn main() {
/// # if false {
/// # let addr = "127.0.0.1:8080".parse().unwrap();
/// ServiceBuilder::new()
///     .resource(MyResource1)
///     .resource(MyResource2)
///     .resource(MyResource3)
///     .run(&addr);
/// # }
/// # }
/// ```
///
/// Defining a middleware stack
///
/// ```rust
/// # #[macro_use] extern crate tower_web;
/// use tower_web::ServiceBuilder;
/// # type FooMiddleware = tower_web::middleware::log::LogMiddleware;
/// # type BarMiddleware = tower_web::middleware::log::LogMiddleware;
///
/// struct MyResource;
///
/// impl_web! {
///     impl MyResource {
///         // ...
///     }
/// }
///
/// # fn main() {
/// # if false {
/// # let addr = "127.0.0.1:8080".parse().unwrap();
/// ServiceBuilder::new()
///     .resource(MyResource)
///     .middleware(FooMiddleware::new("foo"))
///     .middleware(BarMiddleware::new("bar"))
///     .run(&addr);
/// # }
/// # }
/// ```
#[derive(Debug)]
pub struct ServiceBuilder<T, Serializer, Catch, Middleware> {
    /// The inner resource
    resource: T,
    serializer: Serializer,
    catch: Catch,
    middleware: Middleware,
    config: ConfigBuilder,
}

impl ServiceBuilder<(), DefaultSerializer, DefaultCatch, Identity> {
    /// Create a new `ServiceBuilder` with default configuration.
    ///
    /// At least one resource must be added before building the service.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate tower_web;
    /// use tower_web::ServiceBuilder;
    ///
    /// struct MyResource;
    ///
    /// impl_web! {
    ///     impl MyResource {
    ///         // ...
    ///     }
    /// }
    ///
    /// # fn main() {
    /// # if false {
    /// # let addr = "127.0.0.1:8080".parse().unwrap();
    /// ServiceBuilder::new()
    ///     .resource(MyResource)
    ///     .run(&addr);
    /// # }
    /// # }
    /// ```
    pub fn new() -> Self {
        ServiceBuilder {
            resource: (),
            serializer: DefaultSerializer::new(),
            catch: DefaultCatch::new(),
            middleware: Identity::new(),
            config: ConfigBuilder::new(),
        }
    }
}

impl<T, S, C, M> ServiceBuilder<T, S, C, M> {
    /// Add a resource to the service.
    ///
    /// Resources are prioritized based on the order they are added to the
    /// `ServiceBuilder`. If two resources handle the same route, then the one
    /// that was added first gets priority.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate tower_web;
    /// use tower_web::ServiceBuilder;
    ///
    /// struct MyResource;
    ///
    /// impl_web! {
    ///     impl MyResource {
    ///         // ...
    ///     }
    /// }
    ///
    /// # fn main() {
    /// # if false {
    /// # let addr = "127.0.0.1:8080".parse().unwrap();
    /// ServiceBuilder::new()
    ///     .resource(MyResource)
    ///     .run(&addr);
    /// # }
    /// # }
    /// ```
    pub fn resource<U>(self, resource: U)
        -> ServiceBuilder<<T as Chain<U>>::Output, S, C, M>
    where
        T: Chain<U>,
    {
        ServiceBuilder {
            resource: self.resource.chain(resource),
            serializer: self.serializer,
            catch: self.catch,
            middleware: self.middleware,
            config: self.config,
        }
    }

    /// Add a serializer to the service.
    ///
    /// Serializers convert response structs to bytes. Each given serializer
    /// handles a specific content-type. A service may have many registered
    /// serializers, each handling different content-types.
    ///
    /// By default, the service is able to respond with "text/plain" and
    /// "application/json" bodies. Adding new serializers adds the ability to
    /// handle additional formats.
    ///
    /// Currently, the only other supported format is "text/html" and is
    /// provided by the [handlebars] serializer. In future releases, third party
    /// crates will be able to provide serializer implementations.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate tower_web;
    /// use tower_web::ServiceBuilder;
    /// use tower_web::view::Handlebars;
    ///
    /// struct MyResource;
    ///
    /// #[derive(Response)]
    /// #[web(template = "index")]
    /// struct Index {
    ///     title: &'static str,
    /// }
    ///
    /// impl_web! {
    ///     impl MyResource {
    ///         #[get("/")]
    ///         fn index(&self) -> Result<Index, ()> {
    ///             Ok(Index {
    ///                 title: "hello world",
    ///             })
    ///         }
    ///     }
    /// }
    ///
    /// # fn main() {
    /// # if false {
    /// # let addr = "127.0.0.1:0".parse().unwrap();
    /// ServiceBuilder::new()
    ///     .serializer(Handlebars::new())
    ///     .resource(MyResource)
    ///     .run(&addr);
    /// # }
    /// # }
    /// ```
    pub fn serializer<U>(self, serializer: U)
        -> ServiceBuilder<T, <S as Chain<U>>::Output, C, M>
    where
        S: Chain<U>,
    {
        ServiceBuilder {
            resource: self.resource,
            serializer: self.serializer.chain(serializer),
            catch: self.catch,
            middleware: self.middleware,
            config: self.config,
        }
    }

    /// Add a config to the service.
    ///
    /// Configs may be retrieved by their type and used from within extractors.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate tower_web;
    /// use tower_web::ServiceBuilder;
    /// use tower_web::extract::{Extract, Context, Immediate};
    /// use tower_web::util::BufStream;
    ///
    /// struct MyResource;
    /// struct MyConfig {
    ///     foo: String
    /// }
    /// struct MyParam {
    ///     bar: String
    /// }
    ///
    /// impl<B: BufStream> Extract<B> for MyParam {
    ///     type Future = Immediate<MyParam>;
    ///
    ///     fn extract(context: &Context) -> Self::Future {
    ///         let config = context.config::<MyConfig>().unwrap();
    ///         let param = MyParam { bar: config.foo.clone() };
    ///         Immediate::ok(param)
    ///     }
    /// }
    ///
    /// impl_web! {
    ///     impl MyResource {
    ///         #[get("/")]
    ///         fn action(&self, param: MyParam) -> Result<String, ()> {
    ///             Ok(param.bar)
    ///         }
    ///     }
    /// }
    ///
    /// # fn main() {
    /// # if false {
    /// # let addr = "127.0.0.1:0".parse().unwrap();
    /// ServiceBuilder::new()
    ///     .resource(MyResource)
    ///     .config(MyConfig { foo: "bar".to_owned() })
    ///     .run(&addr);
    /// # }
    /// # }
    /// ```
    pub fn config<U>(self, config: U)
                       -> ServiceBuilder<T, S, C, M>
        where
            U: Send + Sync + 'static,
    {
        ServiceBuilder {
            resource: self.resource,
            serializer: self.serializer,
            catch: self.catch,
            middleware: self.middleware,
            config: self.config.insert(config),
        }
    }

    /// Add a middleware to the service.
    ///
    /// Middleware that are defined last will receive requests first. In other
    /// words, when a middleware is added to the `ServiceBuilder`, it will wrap
    /// all previous middeware and all resources.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate tower_web;
    /// use tower_web::ServiceBuilder;
    /// # type FooMiddleware = tower_web::middleware::log::LogMiddleware;
    /// # type BarMiddleware = tower_web::middleware::log::LogMiddleware;
    ///
    /// struct MyResource;
    ///
    /// impl_web! {
    ///     impl MyResource {
    ///         // ...
    ///     }
    /// }
    ///
    /// # fn main() {
    /// # if false {
    /// # let addr = "127.0.0.1:8080".parse().unwrap();
    /// ServiceBuilder::new()
    ///     .resource(MyResource)
    ///     .middleware(FooMiddleware::new("foo"))
    ///     .middleware(BarMiddleware::new("bar"))
    ///     .run(&addr);
    /// # }
    /// # }
    /// ```
    pub fn middleware<U>(self, middleware: U)
        -> ServiceBuilder<T, S, C, <M as Chain<U>>::Output>
    where
        M: Chain<U>,
    {
        ServiceBuilder {
            resource: self.resource,
            serializer: self.serializer,
            catch: self.catch,
            middleware: self.middleware.chain(middleware),
            config: self.config,
        }
    }

    /// Add a global catch handler.
    ///
    /// In the event that a resource responds to a request with an error, the
    /// catch handler has an opportunity to convert that error to a response.
    /// Most of the time, the catch handler is used to convert the error to a
    /// friendy response status.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate tower_web;
    /// extern crate http;
    /// use tower_web::ServiceBuilder;
    ///
    /// struct MyResource;
    ///
    /// impl_web! {
    ///     impl MyResource {
    ///         // ...
    ///     }
    /// }
    ///
    /// # fn main() {
    /// # if false {
    /// # let addr = "127.0.0.1:8080".parse().unwrap();
    /// ServiceBuilder::new()
    ///     .resource(MyResource)
    ///     .catch(|_: &http::Request<()>, error: tower_web::Error| {
    ///         assert!(error.status_code() == http::StatusCode::NOT_FOUND);
    ///
    ///         let response = http::response::Builder::new()
    ///             .status(404)
    ///             .header("content-type", "text/plain")
    ///             .body("where you at?")
    ///             .unwrap();
    ///
    ///         Ok(response)
    ///     })
    ///     .run(&addr);
    /// # }
    /// # }
    /// ```
    pub fn catch<U>(self, catch: U) -> ServiceBuilder<T, S, U, M> {
        ServiceBuilder {
            resource: self.resource,
            serializer: self.serializer,
            catch,
            middleware: self.middleware,
            config: self.config,
        }
    }

    /// Build a `NewWebService` instance
    ///
    /// The returned value impements `tower_service::NewService` and is used to
    /// generate `service::WebService` values. usually, a `NewWebService`
    /// instance is used to generate one service per TCP connection established
    /// to the server.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # #[macro_use] extern crate tower_web;
    /// # extern crate tower_service;
    /// # extern crate futures;
    /// # extern crate http;
    ///
    /// use tower_web::ServiceBuilder;
    /// use tower_service::Service;
    /// use tower_util::MakeService;
    /// use futures::Future;
    ///
    /// struct MyResource;
    ///
    /// impl_web! {
    ///     impl MyResource {
    ///         // ...
    ///     }
    /// }
    ///
    /// # fn main() {
    /// let new_service = ServiceBuilder::new()
    ///     .resource(MyResource)
    ///     .build_new_service();
    ///
    /// // Use `new_service` to get an instance of our web service.
    /// let mut service = new_service.make_service()
    ///     .wait().unwrap();
    ///
    /// // Issue a request to the service
    /// let request = http::request::Builder::new()
    ///     .method("POST")
    ///     .uri("/hello")
    ///     .body("hello".to_string())
    ///     .unwrap();
    ///
    /// let response = service.call(request);
    /// # }
    /// ```
    pub fn build_new_service<RequestBody>(self) -> NewWebService<T::Resource, C::Catch, M>
    where T: IntoResource<S, RequestBody>,
          S: Serializer,
          C: IntoCatch<S>,
          M: HttpMiddleware<RoutedService<T::Resource, C::Catch>>,
          RequestBody: BufStream,
    {
        // Build the routes
        let routes = self.resource.routes();
        let serializer = self.serializer;

        // Create the routed service
        let routed = RoutedService::new(
            self.resource.into_resource(serializer),
            self.catch.into_catch(),
            self.config.into_config(),
            routes);

        NewWebService::new(
            routed,
            self.middleware)
    }

    /// Run the service
    ///
    /// This builds the service and passes it to Hyper to run.
    ///
    /// Note that Hyper requires all types to be `Send`. Thus, for this to work,
    /// all resources must have response types that are `Send`.
    ///
    /// ```rust
    /// # #[macro_use] extern crate tower_web;
    /// use tower_web::ServiceBuilder;
    ///
    /// struct MyResource;
    ///
    /// impl_web! {
    ///     impl MyResource {
    ///         // ...
    ///     }
    /// }
    ///
    /// # fn main() {
    /// # if false {
    /// # let addr = "127.0.0.1:8080".parse().unwrap();
    /// ServiceBuilder::new()
    ///     .resource(MyResource)
    ///     .run(&addr);
    /// # }
    /// # }
    /// ```
    pub fn run(self, addr: &SocketAddr) -> io::Result<()>
    where T: IntoResource<S, crate::run::LiftReqBody>,
          S: Serializer,
          C: IntoCatch<S> + Send + 'static,
          C::Catch: Send,
          M: HttpMiddleware<RoutedService<T::Resource, C::Catch>, RequestBody = crate::run::LiftReqBody> + Send + 'static,
          M::Service: Send,
          <M::Service as HttpService>::Future: Send,
          M::ResponseBody: Send,
          <M::ResponseBody as BufStream>::Item: Send,
          T::Resource: Send + 'static,
          <T::Resource as Resource>::Buf: Send,
          <T::Resource as Resource>::Body: Send,
          <T::Resource as Resource>::Future: Send,
    {
        crate::run::run(addr, self.build_new_service())
    }

    /// Run the service in a non-blocking mode.
    ///
    /// The returned `Future` object must be polled in order to process the incoming requests.
    pub fn serve<I>(self, incoming: I) -> impl Future<Item = (), Error = ()>
    where I: ConnectionStream,
          I::Item: Send + 'static,
          T: IntoResource<S, crate::run::LiftReqBody>,
          S: Serializer,
          C: IntoCatch<S> + Send + 'static,
          C::Catch: Send,
          M: HttpMiddleware<RoutedService<T::Resource, C::Catch>, RequestBody = crate::run::LiftReqBody> + Send + 'static,
          M::Service: Send,
          <M::Service as HttpService>::Future: Send,
          M::ResponseBody: Send,
          <M::ResponseBody as BufStream>::Item: Send,
          T::Resource: Send + 'static,
          <T::Resource as Resource>::Buf: Send,
          <T::Resource as Resource>::Body: Send,
          <T::Resource as Resource>::Future: Send,
    {
        crate::run::serve(incoming, self.build_new_service())
    }
}
