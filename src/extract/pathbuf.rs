use extract::{Context, Error, Extract, ExtractFuture};
use futures::Poll;
use std::path::{self, Path, PathBuf};
use util::buf_stream::BufStream;

impl<B: BufStream> Extract<B> for PathBuf {
    type Future = ExtractPathBuf;

    fn extract(ctx: &Context) -> Self::Future {
        use codegen::Source::*;

        // Get the parameter index from the callsite info
        match ctx.callsite().source() {
            Param(idx) => {
                let path = ctx.request().uri().path();
                let value = ctx.params().get(*idx, path).into();
                ExtractPathBuf(value)
            }
            _ => unimplemented!("A PathBuf can only be extracted from a parameter for now"),
        }
    }
}

pub struct ExtractPathBuf(String);

impl ExtractPathBuf {
    // https://www.owasp.org/index.php/Path_Traversal
    fn check_for_path_traversal(&self) -> Result<(), Error> {
        use self::path::Component::*;

        let path_traversal_error = || Error::invalid_param(&"Path traversal detected");

        let mut depth: u32 = 0;
        for c in Path::new(&self.0).components() {
            match c {
                Prefix(_) | RootDir => {
                    // Escaping to the root is immediately a failure
                    return Err(path_traversal_error());
                }
                CurDir => {
                    // no-op
                }
                ParentDir => {
                    depth = match depth.checked_sub(1) {
                        Some(v) => v,
                        None => return Err(path_traversal_error()),
                    }
                }
                Normal(_) => {
                    depth += 1;
                }
            }
        }

        Ok(())
    }
}

impl ExtractFuture for ExtractPathBuf {
    type Item = PathBuf;

    fn poll(&mut self) -> Poll<(), Error> {
        self.check_for_path_traversal()?;
        Ok(().into())
    }

    fn extract(self) -> Self::Item {
        PathBuf::from(self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::Async;
    use std::path::Path;

    #[test]
    fn extract() {
        let mut extractor = ExtractPathBuf("hello".into());
        let poll_state = extractor.poll().expect("extractor failed");
        assert_eq!(Async::Ready(()), poll_state);
        assert_eq!(Path::new("hello"), extractor.extract());
    }

    #[test]
    fn disallows_path_traversal() {
        let mut extractor = ExtractPathBuf("..".into());
        let poll_err = extractor.poll().unwrap_err();
        assert!(poll_err.is_invalid_param());

        let mut extractor = ExtractPathBuf("a/..".into());
        let poll_state = extractor.poll().expect("extractor failed");
        assert_eq!(Async::Ready(()), poll_state);
        assert_eq!(Path::new("a/.."), extractor.extract());

        let mut extractor = ExtractPathBuf("../a".into());
        let poll_err = extractor.poll().unwrap_err();
        assert!(poll_err.is_invalid_param());

        let mut extractor = ExtractPathBuf("../a/b".into());
        let poll_err = extractor.poll().unwrap_err();
        assert!(poll_err.is_invalid_param());

        let mut extractor = ExtractPathBuf("a/../b".into());
        let poll_state = extractor.poll().expect("extractor failed");
        assert_eq!(Async::Ready(()), poll_state);
        assert_eq!(Path::new("a/../b"), extractor.extract());

        let mut extractor = ExtractPathBuf("a/b/..".into());
        let poll_state = extractor.poll().expect("extractor failed");
        assert_eq!(Async::Ready(()), poll_state);
        assert_eq!(Path::new("a/b/.."), extractor.extract());
    }
}
