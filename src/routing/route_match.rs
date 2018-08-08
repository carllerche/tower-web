use routing::Captures;

use http::Request;

// TODO: Does this live here?
#[derive(Debug)]
pub struct RouteMatch<'a> {
    /// The matched HTTP request head
    request: &'a Request<()>,

    /// Route captures
    captures: Captures,
}

impl<'a> RouteMatch<'a> {
    /// Create a new `RouteMatch`
    pub(crate) fn new(request: &'a Request<()>, captures: Captures) -> Self {
        RouteMatch {
            request,
            captures,
        }
    }

    pub(crate) fn request(&self) -> &Request<()> {
        self.request
    }

    pub(crate) fn captures(&self) -> &Captures {
        &self.captures
    }
}
