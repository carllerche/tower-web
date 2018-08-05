// vary headers

use super::{Config, CorsResource};

use futures::{Async, Future, Poll};
use http::{self, HeaderMap, StatusCode};
use tower_service::Service;
use util::buf_stream::BufStream;

use std::sync::Arc;

pub struct CorsService<S> {
    inner: S,
    config: Arc<Config>,
}

impl<S> CorsService<S> {
    pub(super) fn new(inner: S, config: Arc<Config>) -> CorsService<S> {
        CorsService { inner, config }
    }
}

impl<S, RequestBody, ResponseBody> Service for CorsService<S>
where
    S: Service<Request = http::Request<RequestBody>, Response = http::Response<ResponseBody>>,
    ResponseBody: BufStream,
{
    type Request = http::Request<RequestBody>;
    type Response = http::Response<Option<ResponseBody>>;
    type Error = S::Error;
    type Future = CorsFuture<S::Future>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        // what are we polling
        self.inner.poll_ready()
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        match self.config.process_request(&request) {
            Ok(CorsResource::Preflight(headers)) => CorsFuture::Handled(Some(headers)),
            Ok(CorsResource::Simple(headers)) => {
                CorsFuture::Simple(self.inner.call(request), Some(headers))
            }
            Err(e) => {
                eprintln!("{:?}", e);
                CorsFuture::Handled(None)
            }
        }
    }
}

pub enum CorsFuture<F> {
    Simple(F, Option<HeaderMap>),
    Handled(Option<HeaderMap>),
}

impl<F, ResponseBody> Future for CorsFuture<F>
where
    F: Future<Item = http::Response<ResponseBody>>,
{
    type Item = http::Response<Option<ResponseBody>>;
    type Error = F::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::CorsFuture::*;

        match self {
            Simple(f, headers) => {
                let mut response = try_ready!(f.poll());
                let headers = headers.take().expect("poll called twice");
                response.headers_mut().extend(headers);
                Ok(Async::Ready(response.map(Some)))
            }
            Handled(headers) => {
                let mut response = http::Response::new(None);
                *response.status_mut() = StatusCode::BAD_REQUEST;

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

    use middleware::cors::{AllowedOrigins, CorsBuilder};
    use util::buf_stream::{self, Empty};

    use super::*;

    type TestError = Box<::std::error::Error>;
    type TestResult = ::std::result::Result<(), TestError>;

    #[derive(Debug, Default)]
    struct MockService {
        poll_ready_count: usize,
        requests: Vec<http::Request<()>>,
    }

    impl Service for MockService {
        type Request = http::Request<()>;
        type Response = http::Response<Empty<Option<[u8; 1]>, ()>>;
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
        let cfg = Arc::new(Config::default());
        let mut service = CorsService::new(MockService::default(), cfg);

        service.poll_ready()?;
        assert_eq!(service.inner.poll_ready_count, 1);

        Ok(())
    }

    #[test]
    fn forwards_the_request_when_not_cors() -> TestResult {
        let cfg = Arc::new(Config::default());
        let mut service = CorsService::new(MockService::default(), cfg);
        let req = http::Request::builder().body(())?;

        service.call(req);
        assert_eq!(service.inner.requests.len(), 1);

        Ok(())
    }

    #[test]
    fn does_not_forward_the_request_when_preflight() -> TestResult {
        let cfg = Arc::new(Config::default());
        let mut service = CorsService::new(MockService::default(), cfg);
        let req = http::Request::builder()
            .method(Method::OPTIONS)
            .header(
                header::ORIGIN,
                HeaderValue::from_static("http://test.example"),
            ).header(
                header::ACCESS_CONTROL_REQUEST_METHOD,
                HeaderValue::from_static("POST"),
            ).body(())?;

        service.call(req);
        assert_eq!(service.inner.requests.len(), 0);

        Ok(())
    }

    #[test]
    fn responds_with_error_when_bad_cors() -> TestResult {
        let cfg = Arc::new(Config::default());
        let mut service = CorsService::new(MockService::default(), cfg);
        // Disallowed "Origin" header
        let req = http::Request::builder()
            .header(
                header::ORIGIN,
                HeaderValue::from_static("http://not-me.example"),
            ).body(())?;

        let resp = service.call(req).wait()?;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

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
            ).body(())?;

        let resp = service.call(req).wait()?;
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);

        Ok(())
    }
}
