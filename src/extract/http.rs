//! Permit the extraction of the underlying `http::Request` without the body.

use http::Request;

use crate::extract::{Context, Error, Extract, Immediate};
use crate::util::BufStream;

impl<B: BufStream> Extract<B> for Request<()> {
    type Future = Immediate<Self>;

    fn extract(ctx: &Context<'_>) -> Self::Future {
        let request = Request::builder()
            .version(ctx.request().version())
            .method(ctx.request().method())
            .uri(ctx.request().uri())
            .body(())
            .map_err(|e| Error::invalid_argument(&e))
            .map(|mut request| {
                request
                    .headers_mut()
                    .extend(ctx.request().headers().clone());
                request
            });

        Immediate::result(request)
    }
}
