use super::Params;

#[derive(Debug)]
pub struct RouteMatch {
    /// Extracted route parameters
    params: Params,
}

// ===== impl RouteMatch =====

impl RouteMatch {
    pub(crate) fn new(params: Params) -> Self {
        RouteMatch { params }
    }

    /// Returns the matched parameters
    pub fn params(&self) -> &Params {
        &self.params
    }
}
