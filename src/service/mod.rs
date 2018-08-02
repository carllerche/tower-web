mod builder;
mod http;
mod new_service;
// TODO: Rename this `service`?
mod web;

pub use self::builder::ServiceBuilder;
// TODO: These aren't critical
pub use self::http::{HttpService, NewHttpService, HttpMiddleware, LiftService, LiftMiddleware};
pub use self::new_service::NewWebService;
pub use self::web::WebService;
