extern crate bytes;
#[macro_use]
extern crate futures;
extern crate hyper;
extern crate http;
extern crate tokio;
extern crate tower;
extern crate serde;
extern crate serde_json;

pub mod codegen;

mod builder;
mod resource;
mod response;
mod run;
mod service;

pub use builder::ServiceBuilder;
pub use resource::{Resource, NotFound};
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
    impl_web! => impl_web_impl
}

/*
pub struct Map<T: futures::IntoFuture> {
    inner: T::Future,
}

impl<T: futures::IntoFuture> Map<T> {
    pub fn new(into: T) -> Self {
        Map { inner: into.into_future() }
    }
}

impl<T> futures::Future for Map<T>
where T: futures::IntoFuture<Item = String>,
{
    type Item = http::Response<String>;
    type Error = T::Error;

    fn poll(&mut self) -> futures::Poll<Self::Item, Self::Error> {
        let body = try_ready!(self.inner.poll());
        let resp = http::Response::builder()
            .status(200)
            .header("content-type", "text/plain")
            .body(body).unwrap();

        Ok(resp.into())
    }
}
*/

// ===== end proc_macro_hack junk =====
