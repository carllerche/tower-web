mod builder;
mod http;
mod resource;
mod web;

pub use self::builder::ServiceBuilder;
pub use self::http::HttpService;
pub use self::resource::{Resource, IntoResource};
pub use self::web::WebService;
