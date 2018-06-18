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
        drop((route, request));
        unimplemented!();
    }
}

impl<'a> Extract<'a> for u32 {
    fn extract(route: &RouteMatch, request: &Request<()>) -> Result<Self, ()> {
        drop((route, request));
        unimplemented!();
    }

    fn callsite_extract(
        callsite: &CallSite,
        route_match: &'a RouteMatch,
        request: &'a Request<()>,
    ) -> Result<Self, ()> {
        use std::str::FromStr;

        drop(request);

        // Get the parameter index from the callsite info
        let idx = match callsite.param() {
            Some(idx) => idx,
            None => unimplemented!(),
        };

        let param = match route_match.params().get(idx) {
            Some(param) => param,
            None => return Err(()),
        };

        u32::from_str(param).map_err(|_| ())
    }
}
