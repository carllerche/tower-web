use crate::extract::{Context, Error, Extract, Immediate};
use std::ffi::{OsStr, OsString};
use std::path::{self, Path, PathBuf};
use crate::util::buf_stream::BufStream;

// https://www.owasp.org/index.php/Path_Traversal
fn check_for_path_traversal(path: &Path) -> Result<(), Error> {
    use self::path::Component::*;

    let path_traversal_error = || Error::invalid_argument(&"Path traversal detected");

    let mut depth = 0u32;
    for c in path.components() {
        match c {
            Prefix(_) | RootDir => {
                // Escaping to the root is immediately a failure
                Err(path_traversal_error())?
            }
            CurDir => {
                // no-op
            }
            ParentDir => {
                depth = match depth.checked_sub(1) {
                    Some(v) => v,
                    None => Err(path_traversal_error())?,
                }
            }
            Normal(_) => {
                depth += 1;
            }
        }
    }

    Ok(())
}

fn decode(s: &OsStr) -> Result<PathBuf, Error> {
    let path = PathBuf::from(s);
    check_for_path_traversal(&path)?;
    Ok(path)
}

impl<B: BufStream> Extract<B> for PathBuf {
    type Future = Immediate<Self>;

    fn extract(ctx: &Context) -> Self::Future {
        use crate::extract::ExtractFuture;

        let s = <OsString as Extract<B>>::extract(ctx).extract();
        Immediate::result(decode(&s))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn extract() {
        assert_eq!(
            decode(OsStr::new("hello, world")).unwrap(),
            Path::new("hello, world")
        );
    }

    #[test]
    fn disallows_path_traversal() {
        assert!(decode(OsStr::new("/")).unwrap_err().is_invalid_argument());
        assert!(decode(OsStr::new("..")).unwrap_err().is_invalid_argument());
        assert_eq!(decode(OsStr::new("a/..")).unwrap(), Path::new("a/.."));
        assert!(
            decode(OsStr::new("../a"))
                .unwrap_err()
                .is_invalid_argument()
        );
        assert!(
            decode(OsStr::new("../a/b"))
                .unwrap_err()
                .is_invalid_argument()
        );
        assert_eq!(decode(OsStr::new("a/../b")).unwrap(), Path::new("a/../b"));
        assert_eq!(decode(OsStr::new("a/b/..")).unwrap(), Path::new("a/b/.."));
    }
}
