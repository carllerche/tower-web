//! Types used to extract Serde values from an HTTP request.

use codegen::CallSite;
use extract::{Context, Error, ExtractFuture};
use util::buf_stream::{self, BufStream};

use futures::{Future, Poll};
use serde::de::DeserializeOwned;
use serde_urlencoded;

/*
 * # TODO: Move this module to `codegen`?
 */

/// Extract a value using Serde
pub struct SerdeFuture<T, B> {
    state: State<T, B>,
}

enum State<T, B> {
    Complete(Result<T, Option<Error>>),
    Body(buf_stream::Collect<B, Vec<u8>>),
}

#[doc(hidden)]
pub fn requires_body(callsite: &CallSite) -> bool {
    use codegen::Source::Body;

    match callsite.source() {
        Body => true,
        _ => false,
    }
}

impl<T, B> SerdeFuture<T, B>
where T: DeserializeOwned,
      B: BufStream,
{
    /// Immediately extract a value using only the HTTP request head
    pub fn new_extract(ctx: &Context) -> Self {
        use codegen::Source::*;

        match ctx.callsite().source() {
            Capture(_) => {
                unimplemented!();
            }
            Header(_) => {
                unimplemented!();
            }
            QueryString => {
                let query = ctx.request().uri()
                    .path_and_query()
                    .and_then(|path_and_query| path_and_query.query())
                    .unwrap_or("");

                let res = serde_urlencoded::from_str(query)
                    .map_err(|err| {
                        use std::error::Error as E;

                        if query.is_empty() {
                            Some(Error::missing_argument())
                        } else {
                            Some(Error::invalid_argument(&err.description()))
                        }
                    });

                let state = State::Complete(res);

                SerdeFuture { state }
            }
            Body => {
                unimplemented!();
            }
            Unknown => {
                unimplemented!();
            }
        }
    }

    /// Extract a value using the HTTP request head and body
    pub fn new_extract_body(ctx: &Context, body: B) -> Self {
        use codegen::Source::*;

        match ctx.callsite().source() {
            Capture(_) => {
                unimplemented!();
            }
            Header(_) => {
                unimplemented!();
            }
            QueryString => {
                unimplemented!();
            }
            Body => {
                // TODO: Make content-types pluggable
                assert!({
                    use http::header;

                    ctx.request().headers().get(header::CONTENT_TYPE)
                        .map(|content_type| {
                            content_type.as_bytes().to_ascii_lowercase() ==
                                b"application/json"
                        })
                        .unwrap_or(false)
                });

                let state = State::Body(body.collect());

                SerdeFuture { state }
            }
            Unknown => {
                unimplemented!();
            }
        }
    }
}

impl<T, B> ExtractFuture for SerdeFuture<T, B>
where T: DeserializeOwned,
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

                    let res = try_ready!(res);

                    // And here we deserialize, but we have not determined the
                    // content type yet :(
                    //
                    // TODO: Make content type pluggable
                    ::serde_json::from_slice(&res[..])
                        .map_err(|_| {
                            // TODO: Handle error better
                            Some(Error::internal_error())
                        })
                }
            };

            self.state = State::Complete(res);
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
