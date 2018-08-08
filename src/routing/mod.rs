//! Map HTTP requests to Resource methods.
//!
//! Currently, this module is intended to be used by the `impl_web!` macro and
//! not the end user.

mod builder;
mod captures;
mod path;
mod resource;
mod route;
mod route_match;
mod service;
mod set;

pub use self::builder::Builder;
pub use self::resource::{Resource, IntoResource, Unit};
pub use self::route::Route;
pub use self::route_match::RouteMatch;
pub use self::service::RoutedService;
pub use self::set::RouteSet;

pub(crate) use self::captures::Captures;
pub(crate) use self::path::Path;
