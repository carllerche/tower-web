use crate::extract::{Context, Error, Extract, Immediate};
use percent_encoding;
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::str;
use crate::util::buf_stream::BufStream;

fn osstr_from_bytes(bytes: &[u8]) -> Result<&OsStr, Error> {
    // NOTE: this is too conservative, as we are rejecting valid paths on Unix
    str::from_utf8(bytes)
        .map_err(|e| Error::invalid_argument(&e))
        .map(|s| OsStr::new(s))
}

fn decode(s: &str) -> Result<OsString, Error> {
    let percent_decoded = Cow::from(percent_encoding::percent_decode(s.as_bytes()));
    Ok(osstr_from_bytes(percent_decoded.as_ref())?.to_os_string())
}

impl<B: BufStream> Extract<B> for OsString {
    type Future = Immediate<Self>;

    fn extract(ctx: &Context) -> Self::Future {
        use crate::codegen::Source::*;

        match ctx.callsite().source() {
            Capture(idx) => {
                let path = ctx.request().uri().path();
                let value = ctx.captures().get(*idx, path);

                Immediate::result(decode(value))
            }
            Header(header_name) => {
                let value = match ctx.request().headers().get(header_name) {
                    Some(value) => value,
                    None => {
                        return Immediate::err(Error::missing_argument());
                    }
                };

                let r = value
                    .to_str()
                    .map(OsString::from)
                    .map_err(|e| Error::invalid_argument(&e));
                Immediate::result(r)
            }
            QueryString => {
                let query = ctx.request().uri().query().unwrap_or("");

                Immediate::result(decode(query))
            }
            Body => {
                unimplemented!();
            }
            Unknown => {
                unimplemented!();
            }
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
        assert_eq!(decode("foo").unwrap(), OsString::from("foo"));
        assert_eq!(decode("foo%20bar").unwrap(), OsString::from("foo bar"));
    }
}
