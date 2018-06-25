use service::HttpService;
use util::BufStream;

use futures::Poll;
use http;
use hyper::body::{Body, Chunk, Payload};
use hyper::server::conn::Http;
use hyper::service::Service as HyperService;

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
    type Error = ::Error;

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, Self::Error> {
        self.body.poll()
    }
}

impl BufStream for LiftReqBody {
    type Item = Chunk;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, ::Error> {
        self.body.poll()
            .map_err(|_| ::Error::Internal)
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
    type Error = ::Error;
    type Future = Box<Future<Item = http::Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, request: http::Request<Self::ReqBody>) -> Self::Future {
        let request = request.map(|body| LiftReqBody { body });
        let response = self.inner.call(request)
            .map(|response| {
                response.map(|body| LiftBody { body })
            });

        Box::new(response)
    }
}

/// Run a service
pub fn run<T>(addr: &SocketAddr, service: T) -> io::Result<()>
where
    T: HttpService<RequestBody = LiftReqBody> + Clone + Send + 'static,
    <T::ResponseBody as BufStream>::Item: Send,
    T::ResponseBody: Send,
    T::Future: Send,
{
    let listener = TcpListener::bind(addr)?;
    let http = Arc::new(Http::new());

    tokio::run({
        listener
            .incoming()
            .map_err(|e| println!("failed to accept socket; err = {:?}", e))
            .for_each(move |socket| {
                let h = http.clone();

                tokio::spawn({
                    let service = Lift::new(service.clone());

                    h.serve_connection(socket, service)
                        .map(|_| ())
                        .map_err(|e| {
                            println!("failed to serve connection; err={:?}", e);
                        })
                })
            })
    });

    Ok(())
}
