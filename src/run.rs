use crate::net::{self, ConnectionStream};
use crate::util::BufStream;
use crate::util::http::{HttpService, NewHttpService};

use futures::Poll;
use http;
use http::status::StatusCode;
use hyper;
use hyper::body::{Body, Chunk, Payload};
use hyper::server::conn::Http;
use hyper::service::Service as HyperService;
use crate::util::buf_stream::size_hint::{Builder, SizeHint};

use tokio;
use tokio::net::TcpListener;
use tokio::prelude::*;

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

struct Lift<T: HttpService> {
    inner: T,
}

struct LiftBody<T: HttpService> {
    body: T::ResponseBody,
}

#[derive(Debug)]
pub struct LiftReqBody {
    body: Body,
}

impl<T> Lift<T>
where
    T: HttpService<RequestBody = LiftReqBody>,
{
    fn new(inner: T) -> Self {
        Lift { inner }
    }
}

impl<T> Payload for LiftBody<T>
where
    T: HttpService + 'static,
    <T::ResponseBody as BufStream>::Item: Send,
    T::ResponseBody: Send,
{
    type Data = <T::ResponseBody as BufStream>::Item;
    type Error = crate::Error;

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, Self::Error> {
        self.body.poll()
            .map_err(|_| unimplemented!())
    }
}

impl BufStream for LiftReqBody {
    type Item = Chunk;
    type Error = crate::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, crate::Error> {
        Stream::poll(&mut self.body).map_err(|_| crate::Error::from(StatusCode::INTERNAL_SERVER_ERROR))
    }
}

impl<T> HyperService for Lift<T>
where
    T: HttpService<RequestBody = LiftReqBody> + 'static,
    <T::ResponseBody as BufStream>::Item: Send,
    T::ResponseBody: Send,
    T::Future: Send,
{
    type ReqBody = Body;
    type ResBody = LiftBody<T>;
    type Error = crate::Error;
    type Future = Box<Future<Item = http::Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, request: http::Request<Self::ReqBody>) -> Self::Future {
        let request = request.map(|body| LiftReqBody { body });
        let response = self.inner
            .call_http(request)
            .map(|response| response.map(|body| LiftBody { body }))
            .map_err(|_| unimplemented!())
            ;

        Box::new(response)
    }
}

/// Run a service
pub fn run<T>(addr: &SocketAddr, new_service: T) -> io::Result<()>
where
    T: NewHttpService<RequestBody = LiftReqBody> + Send + 'static,
    T::Future: Send,
    <T::ResponseBody as BufStream>::Item: Send,
    T::ResponseBody: Send,
    T::Service: Send,
    <T::Service as HttpService>::Future: Send,
{
    let listener = TcpListener::bind(addr)?;

    tokio::run(serve(listener.incoming(), new_service));

    Ok(())
}

/// Returns a future that must be polled to process the incoming connections.
///
/// A non-blocking version of `run`.
pub fn serve<S, T>(incoming: S, new_service: T) -> impl Future<Item = (), Error = ()>
where
    S: ConnectionStream,
    S::Item: Send + 'static,
    T: NewHttpService<RequestBody = LiftReqBody> + Send + 'static,
    T::Future: Send,
    <T::ResponseBody as BufStream>::Item: Send,
    T::ResponseBody: Send,
    T::Service: Send,
    <T::Service as HttpService>::Future: Send,
{
    let http = Arc::new(Http::new());
    net::Lift(incoming)
        .map_err(|e| println!("failed to accept socket; err = {:?}", e))
        .for_each(move |socket| {
            let h = http.clone();

            tokio::spawn({
                new_service
                    .new_http_service()
                    .map_err(|_| unimplemented!())
                    .and_then(move |service| {
                        let service = Lift::new(service);

                        h.serve_connection(socket, service)
                            .map(|_| ())
                            .map_err(|e| {
                                println!("failed to serve connection; err={:?}", e);
                            })
                    })
            })
        })
}

impl BufStream for Body {
    type Item = Chunk;
    type Error = hyper::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Stream::poll(self)
    }

    fn size_hint(&self) -> SizeHint {
        let mut builder = Builder::new();
        if let Some(length) = self.content_length() {
            if length < usize::max_value() as u64 {
                let length = length as usize;
                builder.lower(length).upper(length);
            } else {
                builder.lower(usize::max_value());
            }
        }
        builder.build()
    }
}
