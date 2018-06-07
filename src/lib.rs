extern crate bytes;
#[macro_use]
extern crate futures;
extern crate http;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate tokio;
extern crate tower_service;

pub mod codegen;
pub mod resource;
pub mod routing;
pub mod service;

mod builder;
mod error;
mod extract;
mod response;
mod run;

pub use builder::ServiceBuilder;
pub use error::Error;
pub use extract::Extract;
pub use resource::Resource;
pub use response::IntoResponse;
pub use service::Service;

// ===== proc_macro_hack junk =====

#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate tower_web_macros;

#[doc(hidden)]
pub use tower_web_macros::*;

proc_macro_item_decl! {
    /// Implement a Web Service.
    derive_resource! => derive_resource_impl
}

#[macro_export]
macro_rules! impl_web {
    ($($t:tt)*) => {
        $($t)*
        derive_resource! { $($t)* }
    }
}

// ===== end proc_macro_hack junk =====
