use response::{Context, IntoResponse, Serializer, MapErr};
use routing::{RouteSet, RouteMatch};
use service::Resource;
use util::BufStream;
use util::tuple::Either2 as Either;

use bytes::Bytes;
use futures::{Future, Poll};
use http;
use tower_service::Service;

use std::marker::PhantomData;
use std::sync::Arc;

/// Web service
#[derive(Debug)]
pub struct WebService<T, In>
where
    T: Resource,
{
    /// Resource that handles the request
    resource: T,

    /// Route set. Processes request to determine how the resource will process
    /// it.
    routes: Arc<RouteSet<T::Destination>>,

    /*

    /// TODO: Ideally content type negotation would be performed based on the
    /// request. However, this is not implemented yet.
    default_content_type: S::ContentType,
    */

    /// The request body type.
    _p: PhantomData<In>,
}

impl<T: Resource + Clone, In> Clone for WebService<T, In>
where
    T: Resource + Clone,
{
    fn clone(&self) -> WebService<T, In> {
        WebService {
            resource: self.resource.clone(),
            routes: self.routes.clone(),
            _p: PhantomData,
        }
    }
}

/*


type ResponseBody<T> = Either<T, MapErr<Bytes>>;
*/

// ===== impl WebService =====

impl<T, In> WebService<T, In>
where
    T: Resource,
{
    pub(crate) fn new(resource: T, routes: RouteSet<T::Destination>) -> Self {
        let routes = Arc::new(routes);

        WebService {
            resource,
            routes,
            _p: PhantomData,
        }
    }
}

impl<T, In> Service for WebService<T, In>
where
    T: Resource,
    In: BufStream,
{
    type Request = http::Request<In>;
    type Response = <Self::Future as Future>::Item;
    type Error = <Self::Future as Future>::Error;
    type Future = T::Future;
    // type Future = ResponseFuture<T, S>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        // Always ready
        Ok(().into())
    }

    fn call(&mut self, request: Self::Request) -> Self::Future {
        // TODO: Use the body
        let (head, body) = request.into_parts();
        let request = http::Request::from_parts(head, ());

        match self.routes.test(&request) {
            Some((destination, params)) => {
                // Create the `RouteMatch` for the routing result
                let route_match = RouteMatch::new(request, params);

                // Dispatch the requeest
                self.resource
                    .dispatch(destination, route_match, body)
            }
            None => {
                unimplemented!();
                /*
                ResponseFuture {
                    inner: None,
                    serializer,
                    content_type: self.default_content_type.clone(),
                }
                */
            }
        }
    }
}

/*
impl<T, S> Future for ResponseFuture<T, S>
where
    T: Resource,
    S: Serializer,
{
    type Item = http::Response<ResponseBody<T::Body>>;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Self::Item, ::Error> {
        use futures::Async::*;
        use self::Either::*;

        match self.inner {
            Some(ref mut f) => {
                // Get a reference to the serializer. The serializer is what
                // takes the response value from the user and converts it to a
                // binary format.
                let serializer = &*self.serializer;

                // The context tracks all the various factors that are used to
                // determine how a user response is converted to an HTTP
                // response. This context is passed to `IntoResponse::response`.
                let context = Context::new(serializer, &self.content_type);

                let response = match f.poll() {
                    Ok(Ready(response)) => {
                        // Convert to an HTTP response and map the body
                        response.into_response(&context).map(A)
                    }
                    Ok(NotReady) => return Ok(NotReady),
                    Err(e) => {
                        e.into_response()
                            .map(MapErr::new)
                            .map(B)
                    }
                };

                Ok(response.into())
            }
            None => {
                let response = ::Error::from(::ErrorKind::not_found())
                    .into_response()
                    .map(MapErr::new)
                    .map(B);

                Ok(Ready(response))
            }
        }
    }
}
*/
