use super::{Params, Path};

use http::{Method, Request};

/// Requirement on an HTTP request in order to match a route
#[derive(Debug)]
pub(crate) struct Condition {
    /// HTTP method used to match the route
    method: Method,

    /// Path used to match the route
    path: Path,
}

// ===== impl Condition =====

impl Condition {
    /// Create a new condition
    pub fn new(method: Method, path: &str) -> Condition {
        let path = Path::new(path);

        Condition { method, path }
    }

    /// Test a request
    pub fn test(&self, request: &Request<()>) -> Option<Params> {
        if *request.method() != self.method {
            return None;
        }

        self.path.test(request.uri().path())
    }
}
