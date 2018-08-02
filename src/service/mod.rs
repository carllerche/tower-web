mod builder;
mod new_service;
// TODO: Rename this `service`?
mod web;

pub use self::builder::ServiceBuilder;
pub use self::new_service::NewWebService;
pub use self::web::WebService;
