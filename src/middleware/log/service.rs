use futures::{Future, Poll};
use http;
use log::{logger, Level, Record, warn, log_enabled};
use tower_service::Service;

use std::time::Instant;

/// Decorates a service by logging all received requests
#[derive(Debug)]
pub struct LogService<S> {
    inner: S,
    target: &'static str,
}

/// Log the received request once the response has been processed.
#[derive(Debug)]
pub struct ResponseFuture<T> {
    inner: T,
    context: Option<LogContext>,
}

#[derive(Debug)]
struct LogContext {
    method: http::Method,
    path: Option<http::uri::PathAndQuery>,
    version: http::Version,
    start: Instant,
    target: &'static str,
}

impl<S> LogService<S> {
    pub(super) fn new(inner: S, target: &'static str) -> LogService<S> {
        LogService {
            inner,
            target,
        }
    }
}

impl<S, RequestBody, ResponseBody> Service for LogService<S>
where S: Service<Request = http::Request<RequestBody>,
                Response = http::Response<ResponseBody>>,
      S::Error: ::std::error::Error,
{
    type Request = S::Request;
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_ready()
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        let context = if log_enabled!(target: self.target, Level::Info) {
            Some(LogContext {
                method: request.method().clone(),
                path: request.uri().path_and_query().map(|p| p.clone()),
                version: request.version(),
                start: Instant::now(),
                target: self.target,
            })
        } else {
            None
        };

        let inner = self.inner.call(request);

        ResponseFuture {
            inner,
            context,
        }
    }
}

impl<T, B> Future for ResponseFuture<T>
where
    T: Future<Item = http::Response<B>>,
    T::Error: ::std::error::Error,
{
    type Item = T::Item;
    type Error = T::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use futures::Async::*;

        let res = self.inner.poll();

        match (&res, &self.context) {
            (Ok(Ready(ref response)), Some(ref context)) => {
                let full_path = context.path.as_ref()
                    .map(|p| p.as_str())
                    .unwrap_or("/");

                // TODO:
                // - remote_addr
                // - response content length
                // - date
                let status_code = response.status().as_u16();
                let level = match status_code {
                    400..=599 => Level::Error,
                    _ => Level::Info,
                };
                logger().log(&Record::builder()
                    .args(format_args!(
                        "\"{} {} {:?}\" {} {:?}",
                        context.method,
                        full_path,
                        context.version,
                        status_code,
                        context.start.elapsed(),
                    ))
                    .level(level)
                    .target(context.target)
                    .module_path(Some(module_path!()))
                    .file(Some(file!()))
                    .line(Some(line!()))
                    .build());
            }
            (Err(ref err), ..) => {
                warn!("ERROR: {}", err);
            }
            _ => {}
        }

        res
    }
}
