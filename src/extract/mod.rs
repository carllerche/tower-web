mod context;
mod error;
mod immediate;
mod num;
/*
mod option;
mod str;
*/

pub use self::error::Error;
pub use self::context::Context;
pub use self::immediate::Immediate;

use codegen::CallSite;
use routing::RouteMatch;

use futures::Poll;
use http::Request;
// use serde;

/// Extract a value from a request.
pub trait Extract: Sized {
    type Future: ExtractFuture<Item = Self>;

    fn into_future(context: &Context) -> Self::Future;
}

/// Future representing the completion of extracting a value from a request
pub trait ExtractFuture {
    type Item;

    fn poll(&mut self) -> Poll<(), Error>;

    fn extract(self) -> Self::Item;
}

/*
pub struct DeserializeFuture<T> {
    _p: ::std::marker::PhantomData<T>,
}

impl<T> Extract for T
where T: serde::Deserialize<'static>
{
    type Future = DeserializeFuture<T>;

    fn into_future(request: &Context) -> Self::Future {
        unimplemented!();
    }
}

impl<T> ExtractFuture for DeserializeFuture<T> {
    type Item = T;

    fn poll(&mut self) -> Poll<(), Error> {
        unimplemented!();
    }

    fn extract(self, request: &Context) -> Self::Item {
        unimplemented!();
    }
}
*/

/*
/// Extract a value from a request
///
/// This is explicitly not a future. The intent is that any asynchronous
/// operation would happen in a middleware before hand and stash the value in
/// the request extensions.
///
// TODO:
// - Serializer should be passed in
pub trait Extract<'a>: Sized {
    /// TODO: Dox
    fn extract(route: &'a RouteMatch, request: &'a Request<()>) -> Result<Self, Error>;
}

pub trait CallSiteExtract<'a>: Sized {
    /// TODO: Dox
    fn callsite_extract(
        callsite: &CallSite,
        route: &'a RouteMatch,
        request: &'a Request<()>,
    ) -> Result<Self, Error>;
}

impl<'a, T> CallSiteExtract<'a> for T
where T: Extract<'a>
{
    fn callsite_extract(
        _: &CallSite,
        route: &'a RouteMatch,
        request: &'a Request<()>,
    ) -> Result<Self, Error>
    {
        T::extract(route, request)
    }
}

impl<'a, T> CallSiteExtract<'a> for T
where T: serde::Deserialize<'a>,
{
    fn callsite_extract(
        _: &CallSite,
        route: &'a RouteMatch,
        request: &'a Request<()>,
    ) -> Result<Self, Error>
    {
        unimplemented!();
    }
}
*/

pub mod fut {
    use codegen::CallSite;
    use extract::Error;
    use routing::RouteMatch;

    use http::Request;
    use futures::Poll;

    pub trait ExtractFuture<'a> {
        type Item;

        fn poll(&mut self) -> Poll<(), ()>;

        fn extract(&mut self, request: &'a Request<()>) -> Self::Item;
    }

    pub trait IntoExtractFuture<'a>: Sized {
        type Item;
        type Future: ExtractFuture<'a>;

        fn into_future(request: &Request<()>) -> Self::Future;
    }

    // str

    pub struct ExtractStr;

    impl<'a> IntoExtractFuture<'a> for &'a str {
        type Item = &'a str;
        type Future = ExtractStr;

        fn into_future(request: &Request<()>) -> Self::Future {
            unimplemented!();
        }
    }

    impl<'a> ExtractFuture<'a> for ExtractStr {
        type Item = &'a str;

        fn poll(&mut self) -> Poll<(), ()> {
            Ok(().into())
        }

        fn extract(&mut self, request: &'a Request<()>) -> Self::Item {
            unimplemented!();
        }
    }
}
