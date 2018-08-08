use codegen::CallSite;
use routing::{Captures, RouteMatch};

use http::Request;

/// Context available when extracting data from the HTTP request.
///
/// Primarily, `Context` includes a reference to the HTTP request in question.
pub struct Context<'a> {
    /// Reference too the callsite
    callsite: &'a CallSite,

    /// Reference to the HTTP request
    request: &'a Request<()>,

    captures: &'a Captures,
}

impl<'a> Context<'a> {
    // Used as part of codegen, but not part of the public API.
    #[doc(hidden)]
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

    /// Returns a reference to the HTTP request from which the data should be
    /// extracted.
    pub fn request(&self) -> &Request<()> {
        &self.request
    }
}
