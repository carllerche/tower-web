use crate::codegen::CallSite;
use crate::extract::{Context, Error, Extract, ExtractFuture};
use crate::util::buf_stream::{self, BufStream};

use futures::{Future, Poll};

/// Extract a value using `serde`
#[derive(Debug)]
pub struct ExtractBytes<T, B> {
    state: State<T, B>,
}

#[derive(Debug)]
enum State<T, B> {
    Complete(Result<T, Option<Error>>),
    Body(buf_stream::Collect<B, Vec<u8>>),
}

impl<B: BufStream> Extract<B> for Vec<u8> {
    type Future = ExtractBytes<Self, B>;

    fn extract(ctx: &Context) -> Self::Future {
        use crate::codegen::Source::*;

        match ctx.callsite().source() {
            Capture(idx) => {
                let path = ctx.request().uri().path();
                let value = ctx.captures().get(*idx, path)
                    .into();

                let state = State::Complete(Ok(value));
                ExtractBytes { state }
            }
            Header(header_name) => {
                let value = match ctx.request().headers().get(header_name) {
                    Some(value) => value,
                    None => {
                        return ExtractBytes::err(Error::missing_argument());
                    }
                };

                ExtractBytes::ok(value.as_bytes().into())
            }
            QueryString => {
                let query = ctx.request().uri()
                    .path_and_query()
                    .and_then(|path_and_query| path_and_query.query())
                    .unwrap_or("");

                ExtractBytes::ok(query.into())
            }
            Body => {
                panic!("called `extract` but `body` is required");
            }
            Unknown => {
                unimplemented!();
            }
        }
    }

    fn extract_body(ctx: &Context, body: B) -> Self::Future {
        use crate::codegen::Source::*;

        match ctx.callsite().source() {
            Body => {
                let state = State::Body(body.collect());
                ExtractBytes { state }
            }
            _ => panic!("called `extract_body` but not extracting from body"),
        }
    }

    fn requires_body(callsite: &CallSite) -> bool {
        callsite.requires_body()
    }
}

impl<T, B> ExtractBytes<T, B> {
    /// Create an `ExtractBytes` in the completed state
    fn ok(value: T) -> Self {
        let state = State::Complete(Ok(value));
        ExtractBytes { state }
    }

    /// Create an `ExtractBytes` in the error state
    fn err(err: Error) -> Self {
        let state = State::Complete(Err(Some(err)));
        ExtractBytes { state }
    }
}

impl<T, B> ExtractFuture for ExtractBytes<T, B>
where T: From<Vec<u8>>,
      B: BufStream,
{
    type Item = T;

    fn poll(&mut self) -> Poll<(), Error> {
        use self::State::*;

        loop {
            let res = match self.state {
                Complete(Err(ref mut e)) => {
                    return Err(e.take().unwrap());
                }
                Complete(Ok(_)) => {
                    return Ok(().into());
                }
                Body(ref mut collect) => {
                    let res = collect.poll()
                        // TODO: Is there a better way to handle errors?
                        .map_err(|_| Error::internal_error());

                    try_ready!(res).into()
                }
            };

            self.state = State::Complete(Ok(res));
        }
    }

    fn extract(self) -> T {
        use self::State::Complete;

        match self.state {
            Complete(Ok(res)) => res,
            _ => panic!("invalid state"),
        }
    }
}
