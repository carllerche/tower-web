use http::{self, Method};

/// Checks if a route matches a request
pub fn matches<T>(request: &http::Request<T>,
                  method: &Method,
                  path: &str) -> bool
{
    if request.method() != method {
        return false;
    }

    request.uri().path() == path
}
