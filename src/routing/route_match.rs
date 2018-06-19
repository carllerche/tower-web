use super::Params;

#[derive(Debug)]
pub struct RouteMatch<'a> {
    /// Extracted route parameters
    params: Params<'a>,
}

// ===== impl RouteMatch =====

impl<'a> RouteMatch<'a> {
    pub(crate) fn new(params: Params<'a>) -> Self {
        RouteMatch {
            params,
        }
    }

    /// Returns the matched parameters
    pub fn params(&self) -> &Params {
        &self.params
    }
}
