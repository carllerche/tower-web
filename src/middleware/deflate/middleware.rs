use super::DeflateService;
use crate::middleware::Middleware;
use crate::util::buf_stream::BufStream;
use crate::util::buf_stream::deflate::CompressStream;

use flate2::Compression;
use http;
use tower_service::Service;

/// Deflate all response bodies
#[derive(Debug)]
pub struct DeflateMiddleware {
    level: Compression,
}

impl DeflateMiddleware {
    /// Create a new `DeflateMiddleware` instance
    pub fn new(level: Compression) -> DeflateMiddleware {
        DeflateMiddleware { level }
    }
}

impl<S, RequestBody, ResponseBody> Middleware<S> for DeflateMiddleware
where S: Service<Request = http::Request<RequestBody>,
                Response = http::Response<ResponseBody>>,
      RequestBody: BufStream,
      ResponseBody: BufStream,
      S::Error: ::std::error::Error,
{
    type Request = http::Request<RequestBody>;
    type Response = http::Response<CompressStream<ResponseBody>>;
    type Error = S::Error;
    type Service = DeflateService<S>;

    fn wrap(&self, service: S) -> Self::Service {
        DeflateService::new(service, self.level)
    }
}

