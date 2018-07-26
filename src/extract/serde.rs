use codegen::CallSite;
use extract::{Context, Error, ExtractFuture};
use util::BufStream;

use bytes::{BytesMut, BufMut};
use futures::Poll;
use serde::de::DeserializeOwned;
use serde_urlencoded;

/*
 * # TODO: Move this module to `codegen`?
 */

/// Extract a value using `serde`
pub struct SerdeFuture<T, B> {
    state: State<T, B>,
}

enum State<T, B> {
    Complete(Result<T, Option<Error>>),
    Body {
        buffer: BytesMut,
        body: B,
    }
}

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

        // Get the parameter index from the callsite info
        match ctx.callsite().source() {
            Param(_) => {
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
                            Some(Error::missing_param())
                        } else {
                            Some(Error::invalid_param(&err.description()))
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

        // Get the parameter index from the callsite info
        match ctx.callsite().source() {
            Param(_) => {
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

                let buffer = BytesMut::new();
                let state = State::Body { buffer, body };

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
                Body { ref mut buffer, ref mut body } => {
                    loop {
                        let res = body.poll()
                            .map_err(|_| Error::internal_error());

                        match try_ready!(res) {
                            Some(buf) => buffer.put(buf),
                            None => break,
                        }
                    }

                    // And here we deserialize, but we have not determined the
                    // content type yet :(
                    //
                    // TODO: Make content type pluggable
                    ::serde_json::from_slice(&buffer[..])
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
