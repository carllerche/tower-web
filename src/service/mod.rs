mod builder;
mod future;
mod http;
mod new_service;
mod resource;
// TODO: Rename this `service`?
mod web;

pub use self::builder::ServiceBuilder;
// TODO: These aren't critical
pub use self::future::{HttpResponseFuture, IntoHttpFuture};
pub use self::http::HttpService;
pub use self::new_service::NewWebService;
pub use self::resource::{Resource, IntoResource, Unit};
pub use self::web::WebService;
