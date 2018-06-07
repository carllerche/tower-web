use routing::RouteMatch;

use http::Request;

// TODO: This probably should be a future.
pub trait Extract<'a>: Sized {
    /// TODO: Dox
    fn extract(route: &'a RouteMatch, request: &'a Request<()>) -> Result<Self, ()>;

    /// TODO: Dox
    fn named_extract(name: &str, route: &'a RouteMatch, request: &'a Request<()>) -> Result<Self, ()> {
        drop(name);
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
