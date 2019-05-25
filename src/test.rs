pub mod support {
    #![allow(unused_macros, dead_code)]

//    pub use futures;
    use tower_service;

    pub use crate::util::http::HttpService;

    use crate::ServiceBuilder;
    use crate::response::{DefaultSerializer, Serializer};
    use crate::routing::IntoResource;

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

    macro_rules! assert_bad_request {
    ($response:expr) => {
        assert_eq!($response.status(), ::http::StatusCode::BAD_REQUEST)
    }
}

    macro_rules! assert_not_found {
    ($response:expr) => {
        assert_eq!($response.status(), ::http::StatusCode::NOT_FOUND)
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

    macro_rules! assert_internal_error {
    ($response:expr) => {
        assert_eq!($response.status(), ::http::StatusCode::INTERNAL_SERVER_ERROR)
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
        use crate::util::BufStream;
        use crate::test::support::futures::Future;

        let body = $response.into_body().collect().wait().ok().unwrap();
        let body = String::from_utf8(body).unwrap();

        assert_eq!(body, $body)
    }}
}

    pub fn service<U>(resource: U) -> impl TestHttpService<RequestBody = String>
        where U: IntoResource<DefaultSerializer, String>,
    {
        use self::tower_service::NewService;

        ServiceBuilder::new()
            .resource(resource)
            .build_new_service()
            .new_service()
            .wait().unwrap()
    }

    pub fn service_with_serializer<U, S>(resource: U, serializer: S) -> impl TestHttpService<RequestBody = String>
        where
            U: IntoResource<DefaultSerializer<((), S)>, String>,
            S: Serializer,
    {
        use self::tower_service::NewService;

        ServiceBuilder::new()
            .serializer(serializer)
            .resource(resource)
            .build_new_service()
            .new_service()
            .wait().unwrap()
    }

    pub trait TestHttpService: HttpService {
        fn call_unwrap(&mut self, request: http::Request<Self::RequestBody>) -> http::Response<Self::ResponseBody> {
            self.call_http(request).wait().ok().unwrap()
        }
    }

    impl<T: HttpService> TestHttpService for T {
    }
}