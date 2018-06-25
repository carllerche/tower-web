use response::{Context, IntoResponse, Serializer};
use routing::RouteSet;
use service::Resource;
use util::BufStream;

use bytes::Bytes;
use futures::{Future, Stream, Poll};
use http;
use tower_service::Service;

use std::marker::PhantomData;
use std::sync::Arc;

/// Web service
#[derive(Debug)]
pub struct WebService<T, S, In>
where T: Resource,
      S: Serializer,
{
    resource: T,
    routes: Arc<RouteSet<T::Destination, S::ContentType>>,
    serializer: Arc<S>,
    _p: PhantomData<In>,
}

impl<T: Resource + Clone, S, In> Clone for WebService<T, S, In>
where T: Resource + Clone,
      S: Serializer,
{
    fn clone(&self) -> WebService<T, S, In> {
        WebService {
            resource: self.resource.clone(),
            routes: self.routes.clone(),
            serializer: self.serializer.clone(),
            _p: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct ResponseFuture<T, S>
where T: Resource,
      S: Serializer,
{
    inner: Option<T::Future>,
    serializer: Arc<S>,
    content_type: Option<S::ContentType>,
}

#[derive(Debug)]
pub struct ResponseBody<T: Resource> {
    inner: Body<T::Body>,
}

#[derive(Debug)]
enum Body<T> {
    Inner(T),
    NotFound(Option<Bytes>),
}

// ===== impl WebService =====

impl<T, S, In> WebService<T, S, In>
where
    T: Resource,
    S: Serializer,
{
    pub(crate) fn new(resource: T, serializer: S) -> Self {
        let serializer = Arc::new(serializer);
        let routes = Arc::new(resource.routes(&*serializer));

        WebService {
            resource,
            routes,
            serializer,
            _p: PhantomData,
        }
    }
}

impl<T, S, In> Service for WebService<T, S, In>
where
    T: Resource,
    S: Serializer,
    In: BufStream,
{
    type Request = http::Request<In>;
    type Response = http::Response<ResponseBody<T>>;
    type Error = ::Error;
    type Future = ResponseFuture<T, S>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        // Always ready
        Ok(().into())
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        // TODO: Use the body
        let (head, payload) = request.into_parts();
        let request = http::Request::from_parts(head, ());
        let serializer = self.serializer.clone();

        match self.routes.test(&request) {
            Some((destination, content_type, route_match)) => {
                let fut = self.resource.dispatch(
                    destination,
                    &route_match,
                    &request,
                    payload);

                ResponseFuture {
                    inner: Some(fut),
                    serializer,
                    content_type,
                }
            }
            None => ResponseFuture {
                inner: None,
                serializer,
                content_type: None,
            },
        }
    }
}

impl<T, S> Future for ResponseFuture<T, S>
where T: Resource,
      S: Serializer,
{
    type Item = http::Response<ResponseBody<T>>;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Self::Item, ::Error> {
        match self.inner {
            Some(ref mut f) => {
                // Get a reference to the serializer. The serializer is what
                // takes the response value from the user and converts it to a
                // binary format.
                let serializer = &*self.serializer;

                // The content-type determines the serialization format (json,
                // html, etc...). Each route has an associated content-type. The
                // content-type is passed to the serializer and is used to pick
                // the format.
                //
                // TODO: Don't unwrap
                let content_type = self.content_type.as_ref().unwrap();

                // The context tracks all the various factors that are used to
                // determine how a user response is converted to an HTTP
                // response. This context is passed to `IntoResponse::response`.
                let context = Context::new(serializer, content_type);

                // Create the HTTP response.
                let response = try_ready!(f.poll()).into_response(&context);

                // Wrap the response body with `ResponseBody`
                Ok(response
                    .map(|body| ResponseBody {
                        inner: Body::Inner(body),
                    })
                    .into())
            }
            None => {
                let body = ResponseBody {
                    inner: Body::NotFound(Some(Bytes::from_static(b"not found\n"))),
                };

                let response = http::Response::builder()
                    .status(404)
                    .header("content-type", "text/plain")
                    .body(body)
                    .unwrap();

                Ok(response.into())
            }
        }
    }
}

impl<T: Resource> BufStream for ResponseBody<T> {
    type Item = T::Buf;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, ::Error> {
        match self.inner {
            Body::Inner(ref mut s) => s.poll(),
            // Body::NotFound(ref mut stream) => Ok(stream.take().into()),
            _ => unimplemented!(),
        }
    }
}
