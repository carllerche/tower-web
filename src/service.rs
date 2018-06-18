use {Resource, Payload};
use response::IntoResponse;
use routing::RouteSet;

use bytes::Bytes;
use http;
use tokio::prelude::*;
use tower_service;

use std::marker::PhantomData;
use std::sync::Arc;

/// Web service
#[derive(Debug)]
pub struct Service<T: Resource, P> {
    resource: T,
    routes: Arc<RouteSet<T::Destination>>,
    _p: PhantomData<P>,
}

impl<T: Resource + Clone, P> Clone for Service<T, P> {
    fn clone(&self) -> Service<T, P> {
        Service {
            resource: self.resource.clone(),
            routes: self.routes.clone(),
            _p: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct ResponseFuture<T: Resource> {
    inner: Option<T::Future>,
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

// ===== impl Service =====

impl<T, P> Service<T, P>
where
    T: Resource,
{
    pub(crate) fn new(resource: T) -> Self {
        let routes = Arc::new(resource.routes());

        Service {
            resource,
            routes,
            _p: PhantomData,
        }
    }
}

impl<T, P> tower_service::Service for Service<T, P>
where
    T: Resource,
    P: Payload,
{
    type Request = http::Request<P>;
    type Response = http::Response<ResponseBody<T>>;
    type Error = ::Error;
    type Future = ResponseFuture<T>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        // Always ready
        Ok(().into())
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        // TODO: Use the body
        let (head, payload) = request.into_parts();
        let request = http::Request::from_parts(head, ());

        match self.routes.test(&request) {
            Some((destination, route_match)) => {
                let fut = self.resource.dispatch(
                    destination,
                    &route_match,
                    &request,
                    payload);

                ResponseFuture { inner: Some(fut) }
            }
            None => ResponseFuture { inner: None },
        }
    }
}

impl<T: Resource> Future for ResponseFuture<T> {
    type Item = http::Response<ResponseBody<T>>;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Self::Item, ::Error> {
        match self.inner {
            Some(ref mut f) => {
                // Get the inner response
                let response = try_ready!(f.poll()).into_response();

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

impl<T: Resource> Stream for ResponseBody<T> {
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
