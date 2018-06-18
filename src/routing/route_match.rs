use super::Params;

#[derive(Debug)]
pub struct RouteMatch<'a> {
    /// Extracted route parameters
    params: Params<'a>,

    /// Content-type produced by the route
    content_type: Option<&'a str>,
}

// ===== impl RouteMatch =====

impl<'a> RouteMatch<'a> {
    pub(crate) fn new(params: Params<'a>, content_type: Option<&'a str>) -> Self {
        RouteMatch {
            params,
            content_type,
        }
    }

    /// Returns the matched parameters
    pub fn params(&self) -> &Params {
        &self.params
    }

    /// Returns the content-type produced by the route
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_ref()
            .map(|s| &s[..])
    }
}
