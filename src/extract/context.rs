use codegen::CallSite;
use routing::{Params, RouteMatch};

use http::Request;

pub struct Context<'a> {
    /// Reference too the callsite
    callsite: &'a CallSite,

    /// Reference to the HTTP request
    request: &'a Request<()>,

    params: &'a Params,
}

impl<'a> Context<'a> {
    pub fn new(route_match: &'a RouteMatch, callsite: &'a CallSite) -> Context<'a> {
        let request = route_match.request();
        let params = route_match.params();

        Context {
            callsite,
            request,
            params,
        }
    }

    pub(crate) fn callsite(&self) -> &CallSite {
        self.callsite
    }

    pub(crate) fn params(&self) -> &Params {
        self.params
    }

    pub fn request(&self) -> &Request<()> {
        &self.request
    }
}
