//! Define the web service as a set of routes, resources, middlewares, serializers, ...
//!
//! [`ServiceBuilder`] combines all the various components (routes, resources,
//! middlewares, serializers, deserializers, catch handlers, ...) and turns it
//! into an HTTP service.
//!
//! [`ServiceBuilder`]: struct.ServiceBuilder.html

mod builder;
mod new_service;
// TODO: Rename this `service`?
mod web;

pub use self::builder::ServiceBuilder;
pub use self::new_service::NewWebService;
pub use self::web::WebService;
