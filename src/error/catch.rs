use error::Error;
use response::Serializer;
use util::BufStream;

use futures::{future, Future, IntoFuture};
use http;

use std::sync::Arc;

/// Handles an error
pub trait Catch: Clone {

    type Body: BufStream;

    type Future: Future<Item = http::Response<Self::Body>, Error = Error>;

    /// Handles an error
    fn catch(&mut self, request: &http::Request<()>, error: Error) -> Self::Future;
}

pub trait IntoCatch<S> {
    type Catch: Catch;

    fn into_catch(self) -> Self::Catch;
}

#[derive(Debug, Clone)]
pub struct DefaultCatch {
    _p: (),
}

#[derive(Debug)]
pub struct FnCatch<F>(Arc<F>);

// ===== impl DefaultCatch =====

impl DefaultCatch {
    pub fn new() -> DefaultCatch {
        DefaultCatch {
            _p: (),
        }
    }
}

impl<S> IntoCatch<S> for DefaultCatch
where S: Serializer,
{
    type Catch = DefaultCatch;

    fn into_catch(self) -> Self::Catch {
        self
    }
}

impl Catch for DefaultCatch {
    type Body = &'static str;
    type Future = future::FutureResult<http::Response<Self::Body>, Error>;

    fn catch(&mut self, _request: &http::Request<()>, error: Error) -> Self::Future {
        println!("ERROR = {:?}", error);
        // TODO: Improve the default errors
        let (status, msg) = if error.kind().is_not_found() {
            (404, "not found")
        } else if error.kind().is_bad_request() {
            (400, "bad request")
        } else {
            (500, "internal server error")
        };

        let response = http::response::Builder::new()
            .status(status)
            .header("content-type", "text/plain")
            .body(msg)
            .unwrap();

        future::ok(response)
    }
}

// ===== impl FnCatch =====

impl<F, R, Body, S> IntoCatch<S> for F
where F: Fn(&http::Request<()>, Error) -> R,
      R: IntoFuture<Item = http::Response<Body>, Error = Error>,
      Body: BufStream,
{
    type Catch = FnCatch<F>;

    fn into_catch(self) -> Self::Catch {
        FnCatch(Arc::new(self))
    }
}

impl<F, R, Body> Catch for FnCatch<F>
where F: Fn(&http::Request<()>, Error) -> R,
      R: IntoFuture<Item = http::Response<Body>, Error = Error>,
      Body: BufStream,
{
    type Body = Body;
    type Future = R::Future;

    fn catch(&mut self, request: &http::Request<()>, error: Error) -> Self::Future {
        self.0(request, error).into_future()
    }
}

impl<F> Clone for FnCatch<F> {
    fn clone(&self) -> FnCatch<F> {
        FnCatch(self.0.clone())
    }
}
