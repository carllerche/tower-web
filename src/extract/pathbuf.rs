use extract::{Context, Error, Extract, Immediate};
use percent_encoding;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::{self, Path, PathBuf};
use util::buf_stream::BufStream;

#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
fn osstr_from_bytes(bytes: &[u8]) -> Result<&OsStr, Error> {
    use std::os::unix::ffi::OsStrExt;
    Ok(OsStr::from_bytes(bytes))
}

#[cfg(any(target_os = "windows", target_arch = "wasm32"))]
fn osstr_from_bytes(bytes: &[u8]) -> Result<&OsStr, Error> {
    use std::str;
    str::from_utf8(bytes)
        .map_err(|e| Error::invalid_argument(&e))
        .map(|s| OsStr::new(s))
}

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

fn decode(s: &str) -> Result<PathBuf, Error> {
    let percent_decoded = Cow::from(percent_encoding::percent_decode(s.as_bytes()));
    let path = PathBuf::from(osstr_from_bytes(percent_decoded.as_ref())?);

    check_for_path_traversal(&path)?;
    Ok(path)
}

impl<B: BufStream> Extract<B> for PathBuf {
    type Future = Immediate<PathBuf>;

    fn extract(ctx: &Context) -> Self::Future {
        use codegen::Source::*;

        match ctx.callsite().source() {
            Capture(idx) => {
                let path = ctx.request().uri().path();
                let value = ctx.captures().get(*idx, path);

                Immediate::result(decode(value))
            }
            _ => unimplemented!("A PathBuf can only be extracted from a path capture for now"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn extract() {
        assert_eq!(Path::new("hello, world"), decode("hello,%20world").unwrap());
    }

    #[test]
    fn disallows_path_traversal() {
        assert!(decode("/").unwrap_err().is_invalid_argument());
        assert!(decode("..").unwrap_err().is_invalid_argument());
        assert_eq!(decode("a/..").unwrap(), Path::new("a/.."));
        assert!(decode("../a").unwrap_err().is_invalid_argument());
        assert!(decode("../a/b").unwrap_err().is_invalid_argument());
        assert_eq!(decode("a/../b").unwrap(), Path::new("a/../b"));
        assert_eq!(decode("a/b/..").unwrap(), Path::new("a/b/.."));
        assert!(decode("%2e%2e").unwrap_err().is_invalid_argument());
        assert_eq!(decode("a/%2e%2e").unwrap(), Path::new("a/.."));
    }
}
