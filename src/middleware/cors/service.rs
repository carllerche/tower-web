use super::{Config, CorsResource};

use futures::{Async, Future, Poll};
use http::{self, HeaderMap, Request, Response, StatusCode};
use tower_service::Service;
use crate::util::http::HttpService;

use std::sync::Arc;

/// Decorates a service, providing an implementation of the CORS specification.
#[derive(Debug)]
pub struct CorsService<S> {
    inner: S,
    config: Arc<Config>,
}

impl<S> CorsService<S> {
    pub(super) fn new(inner: S, config: Arc<Config>) -> CorsService<S> {
        CorsService { inner, config }
    }
}

impl<S> Service for CorsService<S>
where
    S: HttpService,
{
    type Request = Request<S::RequestBody>;
    type Response = Response<Option<S::ResponseBody>>;
    type Error = S::Error;
    type Future = CorsFuture<S::Future>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_http_ready()
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        let inner = match self.config.process_request(&request) {
            Ok(CorsResource::Preflight(headers)) => CorsFutureInner::Handled(Some(headers)),
            Ok(CorsResource::Simple(headers)) => {
                CorsFutureInner::Simple(self.inner.call_http(request), Some(headers))
            }
            Err(e) => {
                debug!("CORS request to {} is denied: {:?}", request.uri(), e);
                CorsFutureInner::Handled(None)
            }
        };

        CorsFuture(inner)
    }
}

#[derive(Debug)]
pub struct CorsFuture<F>(CorsFutureInner<F>);

impl<F, ResponseBody> Future for CorsFuture<F>
where
    F: Future<Item = http::Response<ResponseBody>>,
{
    type Item = http::Response<Option<ResponseBody>>;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

#[derive(Debug)]
enum CorsFutureInner<F> {
    Simple(F, Option<HeaderMap>),
    Handled(Option<HeaderMap>),
}

impl<F, ResponseBody> Future for CorsFutureInner<F>
where
    F: Future<Item = http::Response<ResponseBody>>,
{
    type Item = http::Response<Option<ResponseBody>>;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::CorsFutureInner::*;

        match self {
            Simple(f, headers) => {
                let mut response = try_ready!(f.poll());
                let headers = headers.take().expect("poll called twice");
                response.headers_mut().extend(headers);
                Ok(Async::Ready(response.map(Some)))
            }
            Handled(headers) => {
                let mut response = http::Response::new(None);
                *response.status_mut() = StatusCode::FORBIDDEN;

                if let Some(headers) = headers.take() {
                    *response.status_mut() = StatusCode::NO_CONTENT;
                    *response.headers_mut() = headers;
                }

                Ok(Async::Ready(response))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use futures::future::{self, FutureResult};
    use http::{
        header::{self, HeaderValue},
        Method,
    };

    use crate::middleware::cors::{AllowedOrigins, CorsBuilder};
    use crate::util::buf_stream::{self, Empty};

    use super::*;

    type TestError = Box<::std::error::Error>;
    type TestResult = ::std::result::Result<(), TestError>;

    type DontCare = Empty<Option<[u8; 1]>, ()>;

    #[derive(Debug, Default)]
    struct MockService {
        poll_ready_count: usize,
        requests: Vec<http::Request<DontCare>>,
    }

    impl Service for MockService {
        type Request = http::Request<DontCare>;
        type Response = http::Response<DontCare>;
        type Error = TestError;
        type Future = FutureResult<Self::Response, Self::Error>;

        fn poll_ready(&mut self) -> Poll<(), Self::Error> {
            self.poll_ready_count += 1;
            Ok(Async::Ready(()))
        }

        fn call(&mut self, request: Self::Request) -> Self::Future {
            self.requests.push(request);
            future::ok(http::Response::new(buf_stream::empty()))
        }
    }

    #[test]
    fn polls_the_inner_service() -> TestResult {
        let cfg = Arc::new(CorsBuilder::new().into_config());
        let mut service = CorsService::new(MockService::default(), cfg);

        service.poll_ready()?;
        assert_eq!(service.inner.poll_ready_count, 1);

        Ok(())
    }

    #[test]
    fn forwards_the_request_when_not_cors() -> TestResult {
        let cfg = Arc::new(CorsBuilder::new().into_config());
        let mut service = CorsService::new(MockService::default(), cfg);
        let req = http::Request::builder().body(buf_stream::empty())?;

        service.call(req);
        assert_eq!(service.inner.requests.len(), 1);

        Ok(())
    }

    #[test]
    fn does_not_forward_the_request_when_preflight() -> TestResult {
        let cfg = Arc::new(CorsBuilder::new().into_config());
        let mut service = CorsService::new(MockService::default(), cfg);
        let req = http::Request::builder()
            .method(Method::OPTIONS)
            .header(
                header::ORIGIN,
                HeaderValue::from_static("http://test.example"),
            ).header(
                header::ACCESS_CONTROL_REQUEST_METHOD,
                HeaderValue::from_static("POST"),
            ).body(buf_stream::empty())?;

        service.call(req);
        assert_eq!(service.inner.requests.len(), 0);

        Ok(())
    }

    #[test]
    fn responds_with_error_when_bad_cors() -> TestResult {
        let cfg = Arc::new(CorsBuilder::new().into_config());
        let mut service = CorsService::new(MockService::default(), cfg);
        // Disallowed "Origin" header
        let req = http::Request::builder()
            .header(
                header::ORIGIN,
                HeaderValue::from_static("http://not-me.example"),
            ).body(buf_stream::empty())?;

        let resp = service.call(req).wait()?;
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);

        Ok(())
    }

    #[test]
    fn responds_with_no_content_when_ok_preflight() -> TestResult {
        let cfg = CorsBuilder::new()
            .allow_origins(AllowedOrigins::Any { allow_null: false })
            .allow_methods(vec![Method::POST])
            .into_config();

        let mut service = CorsService::new(MockService::default(), Arc::new(cfg));
        let req = http::Request::builder()
            .method(Method::OPTIONS)
            .header(
                header::ACCESS_CONTROL_REQUEST_METHOD,
                HeaderValue::from_static("POST"),
            ).header(
                header::ORIGIN,
                HeaderValue::from_bytes(b"http://test.example")?,
            ).body(buf_stream::empty())?;

        let resp = service.call(req).wait()?;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        Ok(())
    }
}
