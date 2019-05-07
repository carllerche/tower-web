use crate::codegen::CallSite;
use crate::extract::{Context, Error, Extract, ExtractFuture};
use crate::extract::bytes::ExtractBytes;
use percent_encoding;
use std::borrow::Cow;
use crate::util::BufStream;

use futures::{Poll, Async};

#[derive(Debug)]
pub struct ExtractString<B> {
    inner: Option<ExtractBytes<Vec<u8>, B>>,
    decode: bool,
    item: Option<String>,
}

impl<B: BufStream> Extract<B> for String {
    type Future = ExtractString<B>;

    fn extract(ctx: &Context) -> Self::Future {
        use crate::codegen::Source::*;

        let inner = Vec::extract(ctx);

        match ctx.callsite().source() {
            Capture(_) | QueryString => {
                ExtractString {
                    inner: Some(inner),
                    decode: true,
                    item: None,
                }
            }
            _ => {
                ExtractString {
                    inner: Some(inner),
                    decode: false,
                    item: None,
               }
            }
        }
    }

    fn extract_body(ctx: &Context, body: B) -> Self::Future {
        ExtractString {
            inner: Some(Vec::extract_body(ctx, body)),
            decode: false,
            item: None,
        }
    }

    fn requires_body(callsite: &CallSite) -> bool {
        <Vec<u8> as Extract<B>>::requires_body(callsite)
    }
}

impl<B> ExtractFuture for ExtractString<B>
where
    B: BufStream,
{
    type Item = String;

    fn poll(&mut self) -> Poll<(), Error> {
        try_ready!(self.inner.as_mut().unwrap().poll());

        let bytes = self.inner.take().unwrap().extract();

        let mut string = String::from_utf8(bytes)
            .map_err(|_| {
                Error::invalid_argument(&"invalid UTF-8 string")
            })?;

        if self.decode {
            string = decode(&string)?;
        }

        self.item = Some(string);

        Ok(Async::Ready(()))
    }

    fn extract(self) -> String {
        self.item.unwrap()
    }
}

fn decode(s: &str) -> Result<String, Error> {
    percent_encoding::percent_decode(s.as_bytes())
        .decode_utf8()
        .map(Cow::into_owned)
        .map_err(|e| Error::invalid_argument(&e))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extract() {
        assert_eq!("hello, world", decode("hello,%20world").unwrap());
        assert!(decode("%ff").unwrap_err().is_invalid_argument());
    }
}
