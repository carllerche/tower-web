use call_site::CallSite;
use routing::RouteMatch;

use http::Request;

// TODO: This probably should be a future.
pub trait Extract<'a>: Sized {
    /// TODO: Dox
    fn extract(route: &'a RouteMatch, request: &'a Request<()>) -> Result<Self, ()>;

    /// TODO: Dox
    fn callsite_extract(
        callsite: &CallSite,
        route: &'a RouteMatch,
        request: &'a Request<()>,
    ) -> Result<Self, ()> {
        drop(callsite);
        Self::extract(route, request)
    }
}

impl<'a> Extract<'a> for String {
    fn extract(route: &RouteMatch, request: &Request<()>) -> Result<Self, ()> {
        drop((route, request));
        unimplemented!();
    }
}

impl<'a> Extract<'a> for &'a str {
    fn extract(route: &'a RouteMatch, request: &Request<()>) -> Result<Self, ()> {
        drop(request);
        Ok(&route.params()[0])
    }
}

impl<'a> Extract<'a> for u32 {
    fn extract(route: &RouteMatch, request: &Request<()>) -> Result<Self, ()> {
        drop((route, request));
        unimplemented!();
    }

    fn callsite_extract(
        callsite: &CallSite,
        route: &'a RouteMatch,
        request: &'a Request<()>,
    ) -> Result<Self, ()> {
        // TODO: Implement
        println!("CALL SITE = {:?}", callsite);
        Ok(123)
    }
}
