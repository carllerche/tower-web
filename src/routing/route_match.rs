use routing::Params;

use http::Request;

// TODO: Does this live here?
#[derive(Debug)]
pub struct RouteMatch<'a> {
    /// The matched HTTP request head
    request: &'a Request<()>,

    /// Extracted route parameters
    params: Params,
}

impl<'a> RouteMatch<'a> {
    /// Create a new `RouteMatch`
    pub(crate) fn new(request: &'a Request<()>, params: Params) -> Self {
        RouteMatch {
            request,
            params,
        }
    }

    pub(crate) fn request(&self) -> &Request<()> {
        self.request
    }

    pub(crate) fn params(&self) -> &Params {
        &self.params
    }
}
