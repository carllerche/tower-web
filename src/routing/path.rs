use super::Captures;

#[derive(Debug)]
pub(crate) struct Path {
    segments: Vec<Segment>,
}

#[derive(Debug)]
enum Segment {
    Literal(String),
    Capture,
    Glob,
}

impl Path {
    /// Create a new path condition
    pub fn new(mut path: &str) -> Path {
        if path.ends_with("/") {
            path = &path[0..path.len() - 1];
        }

        let segments = path.split("/")
            .map(|segment| {
                let c = segment.chars().next();

                if c == Some(':') {
                    Segment::Capture
                } else if c == Some('*') {
                    Segment::Glob
                } else {
                    Segment::Literal(segment.to_string())
                }
            })
            .collect();

        Path { segments }
    }

    /// Test the path component of a request
    pub fn test(&self, mut path: &str) -> Option<Captures> {
        if path.ends_with("/") {
            path = &path[0..path.len() - 1];
        }

        let mut i = 0;
        let mut captures = vec![];
        let base = path.as_ptr() as usize;

        for segment in path.split("/") {
            if i == self.segments.len() {
                return None;
            }

            match self.segments[i] {
                Segment::Capture => {
                    let ptr = segment.as_ptr() as usize;

                    captures.push((ptr - base, segment.len()));
                }
                Segment::Glob => {
                    let ptr = segment.as_ptr() as usize;
                    let start_offset = ptr - base;
                    let len = path.len() - start_offset;
                    captures.push((start_offset, len));
                    i += 1;
                    break;
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

        Some(Captures::new(captures))
    }
}

#[test]
fn test_segments() {
    let slash = Path::new("/");

    assert!(slash.test("/").is_some());
    assert!(slash.test("/foo").is_none());
    assert!(slash.test("//").is_none());

    let one_lit = Path::new("/foo");

    assert!(one_lit.test("/foo").is_some());
    assert!(one_lit.test("/").is_none());
    assert!(one_lit.test("/bar").is_none());
    assert!(one_lit.test("/foo/bar").is_none());

    let non_ascii = Path::new("/£");

    assert!(non_ascii.test("/£").is_some());
    assert!(non_ascii.test("/").is_none());
    assert!(non_ascii.test("/bar").is_none());

    let capture = Path::new("/:id");
    let captures = capture.test("/foo").unwrap();
    assert_eq!(captures.len(), 1);
    assert_eq!(captures.get(0, "/foo"), "foo");

    assert!(capture.test("/").is_none());

    let multi = Path::new("/:one/:two");
    let captures = multi.test("/foo/bar").unwrap();
    assert_eq!(captures.len(), 2);
    assert_eq!(captures.get(0, "/foo/bar"), "foo");
    assert_eq!(captures.get(1, "/foo/bar"), "bar");
}


#[test]
fn test_glob_segments() {
    let glob = Path::new("/*foo");

    let path = "/alpha";
    let captures = glob.test(path).unwrap();
    assert_eq!(captures.len(), 1);
    assert_eq!(captures.get(0, path), "alpha");

    let path = "/beta";
    let captures = glob.test(path).unwrap();
    assert_eq!(captures.len(), 1);
    assert_eq!(captures.get(0, path), "beta");

    let path = "/alpha/beta";
    let captures = glob.test(path).unwrap();
    assert_eq!(captures.len(), 1);
    assert_eq!(captures.get(0, path), "alpha/beta");

    let path = "/alpha/beta/gamma";
    let captures = glob.test(path).unwrap();
    assert_eq!(captures.len(), 1);
    assert_eq!(captures.get(0, path), "alpha/beta/gamma");
}

#[test]
fn test_glob_and_capture_segments() {
    let glob = Path::new("/:id/*foo");

    assert!(glob.test("/42").is_none());
    assert!(glob.test("/42/").is_none());

    let path = "/42/a";
    let captures = glob.test(path).unwrap();
    assert_eq!(captures.len(), 2);
    assert_eq!(captures.get(0, path), "42");
    assert_eq!(captures.get(1, path), "a");

    let path = "/42/a/b";
    let captures = glob.test(path).unwrap();
    assert_eq!(captures.len(), 2);
    assert_eq!(captures.get(0, path), "42");
    assert_eq!(captures.get(1, path), "a/b");
}
