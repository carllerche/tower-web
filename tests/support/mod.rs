#![allow(unused_macros)]

pub use tower_web::service::HttpService;

use tower_web::ServiceBuilder;
use tower_web::response::DefaultSerializer;
use tower_web::service::IntoResource;

pub use futures::Future;
use http;

macro_rules! get {
    ($uri:expr) => {
        ::http::request::Builder::new()
            .uri($uri)
            .body("".to_string())
            .unwrap()
    };
    ($uri:expr, $($k:tt: $v:tt),*) => {
        ::http::request::Builder::new()
            .uri($uri)
            $(.header($k, $v))*
            .body("".to_string())
            .unwrap()
    };
}

macro_rules! post {
    ($uri:expr, $body:expr) => {
        ::http::request::Builder::new()
            .method("POST")
            .uri($uri)
            .body($body.to_string())
            .unwrap()
    };
    ($uri:expr, $body:expr, $($k:tt: $v:tt),*) => {
        ::http::request::Builder::new()
            .method("POST")
            .uri($uri)
            $(.header($k, $v))*
            .body($body.to_string())
            .unwrap()
    };
}

macro_rules! assert_ok {
    ($response:expr) => {
        assert_eq!($response.status(), ::http::StatusCode::OK)
    }
}

macro_rules! assert_created {
    ($response:expr) => {
        assert_eq!($response.status(), ::http::StatusCode::CREATED)
    }
}

macro_rules! assert_accepted {
    ($response:expr) => {
        assert_eq!($response.status(), ::http::StatusCode::ACCEPTED)
    }
}

macro_rules! assert_header {
    ($response:expr, $name:expr, $value:expr) => {{
        let n = $name;

        let actual = match $response.headers().get(n) {
            Some(v) => v,
            None => panic!("missing header {:?}", n),
        };

        assert_eq!(actual, $value)
    }}
}

macro_rules! assert_body {
    ($response:expr, $body:expr) => {{
        use ::tower_web::util::BufStream;
        use ::futures::Future;

        let body = $response.into_body().collect().wait().unwrap();
        let body = String::from_utf8(body).unwrap();

        assert_eq!(body, $body)
    }}
}

pub fn service<U>(resource: U) -> impl TestHttpService<RequestBody = String>
where U: IntoResource<DefaultSerializer>,
{
    ServiceBuilder::new()
        .resource(resource)
        .build()
}

pub trait TestHttpService: HttpService {
    fn call_unwrap(&mut self, request: http::Request<Self::RequestBody>) -> http::Response<Self::ResponseBody> {
        self.call(request).wait().unwrap()
    }
}

impl<T: HttpService> TestHttpService for T {
}
