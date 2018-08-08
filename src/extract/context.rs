use codegen::CallSite;
use routing::{Captures, RouteMatch};

use http::Request;

pub struct Context<'a> {
    /// Reference too the callsite
    callsite: &'a CallSite,

    /// Reference to the HTTP request
    request: &'a Request<()>,

    captures: &'a Captures,
}

impl<'a> Context<'a> {
    pub fn new(route_match: &'a RouteMatch, callsite: &'a CallSite) -> Context<'a> {
        let request = route_match.request();
        let captures = route_match.captures();

        Context {
            callsite,
            request,
            captures,
        }
    }

    pub(crate) fn callsite(&self) -> &CallSite {
        self.callsite
    }

    pub(crate) fn captures(&self) -> &Captures {
        self.captures
    }

    pub fn request(&self) -> &Request<()> {
        &self.request
    }
}
