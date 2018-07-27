use super::Params;

use http::{Method, Request};

/// Requirement on an HTTP request in order to match a route
#[derive(Debug)]
pub(crate) struct Condition {
    /// HTTP method used to match the route
    method: Method,

    /// Path used to match the route
    path: Segments,
}

#[derive(Debug)]
struct Segments {
    segments: Vec<Segment>,
}

#[derive(Debug)]
enum Segment {
    Literal(String),
    Param,
}

// ===== impl Condition =====

impl Condition {
    /// Create a new condition
    pub fn new(method: Method, path: &str) -> Condition {
        let path = Segments::new(path);

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

// ===== impl Segments =====

impl Segments {
    /// Create a new condition
    pub fn new(mut path: &str) -> Segments {
        if !path.is_empty() && &path[path.len() - 1..path.len()] == "/" {
            path = &path[0..path.len() - 1];
        }

        let segments = path.split("/")
            .map(|segment| {
                if segment.chars().next() == Some(':') {
                    Segment::Param
                } else {
                    Segment::Literal(segment.to_string())
                }
            })
            .collect();

        Segments { segments }
    }

    /// Test the path component of a request
    fn test(&self, mut path: &str) -> Option<Params> {
        if path.ends_with("/") {
            path = &path[0..path.len() - 1];
        }

        let mut i = 0;
        let mut params = vec![];
        let base = path.as_ptr() as usize;

        for segment in path.split("/") {
            if i == self.segments.len() {
                return None;
            }

            match self.segments[i] {
                Segment::Param => {
                    let ptr = segment.as_ptr() as usize;

                    params.push((ptr - base, segment.len()));
                }
                Segment::Literal(ref val) => {
                    if segment != val {
                        return None;
                    }
                }
            }

            i += 1;
        }

        if i != self.segments.len() {
            return None;
        }

        Some(Params::new(params))
    }
}

#[test]
fn test_segments() {
    let slash = Segments::new("/");

    assert!(slash.test("/").is_some());
    assert!(slash.test("/foo").is_none());
    assert!(slash.test("//").is_none());

    let one_lit = Segments::new("/foo");

    assert!(one_lit.test("/foo").is_some());
    assert!(one_lit.test("/").is_none());
    assert!(one_lit.test("/bar").is_none());
    assert!(one_lit.test("/foo/bar").is_none());

    let capture = Segments::new("/:id");
    let params = capture.test("/foo").unwrap();
    assert_eq!(params.len(), 1);
    assert_eq!(params.get(0, "/foo"), "foo");

    assert!(capture.test("/").is_none());

    let multi = Segments::new("/:one/:two");
    let params = multi.test("/foo/bar").unwrap();
    assert_eq!(params.len(), 2);
    assert_eq!(params.get(0, "/foo/bar"), "foo");
    assert_eq!(params.get(1, "/foo/bar"), "bar");
}
