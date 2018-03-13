#[macro_use]
extern crate futures;
extern crate http;
extern crate tower;

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

// ===== end proc_macro_hack junk =====

pub mod codegen {
    //! Types re-exported by the library for use in codegen

    pub mod tower {
        //! Types provided by the `tower` crate

        pub use ::tower::Service;
    }

    pub mod http {
        //! Types provided by the `http` crate.
        pub use ::http::{Request, Response};
    }

    pub mod futures {
        //! Types provided by the `futures` crate

        pub use ::futures::{Future, IntoFuture, Poll, Async};
    }
}
