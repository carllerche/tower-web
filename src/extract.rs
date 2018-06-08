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
        use std::str::{self, FromStr};

        // Get the parameter index from the callsite info
        let idx = match callsite.param() {
            Some(idx) => idx,
            None => unimplemented!(),
        };

        let raw = match route_match.params().get(idx) {
            Some(raw) => raw,
            None => return Err(()),
        };

        let s = match str::from_utf8(raw) {
            Ok(s) => s,
            Err(_) => return Err(()),
        };

        u32::from_str(s)
            .map_err(|_| ())
    }
}
