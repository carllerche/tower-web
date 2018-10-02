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
#[derive(Debug)]
pub struct SerdeFuture<T, B> {
    state: State<T, B>,
    is_json: bool,
}

#[derive(Debug)]
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

                SerdeFuture { state, is_json: false }
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
                unimplemented!("Capture");
            }
            Header(_) => {
                unimplemented!("Header");
            }
            QueryString => {
                unimplemented!("QueryString");
            }
            Body => {
                use http::header;

                if let Some(value) = ctx.request().headers().get(header::CONTENT_TYPE) {
                    let content_type = value.as_bytes().to_ascii_lowercase();

                    match &content_type[..] {
                        b"application/json" => {
                            let state = State::Body(body.collect());

                            SerdeFuture { state, is_json: true }  
                        }
                        b"application/x-www-form-urlencoded" => {
                            let state = State::Body(body.collect());
                            
                            SerdeFuture { state, is_json: false }   
                        }
                        _ => panic!("Unknown content type")
                    } 
                } else {
                    panic!("Content-Type header not present")
                }
            }
            Unknown => {
                unimplemented!("Unknown");
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
                    
                    if self.is_json == true {
                        ::serde_json::from_slice(&res[..])
                            .map_err(|_| {
                                // TODO: Handle error better
                                Some(Error::internal_error())
                        })
                    } else {
                        ::serde_urlencoded::from_bytes(&res[..])
                            .map_err(|_| {
                                // TODO: Handle error better
                                Some(Error::internal_error())
                        })
                    }
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
