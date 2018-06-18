use {Resource, Service};
use service::ResponseBody;

use futures::Poll;
use http;
use hyper::body::{Body, Chunk, Payload};
use hyper::server::conn::Http;
use hyper::service::Service as HyperService;
// use hyper::server::{Http, Service as HyperService};

use tokio;
use tokio::net::TcpListener;
use tokio::prelude::*;

use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

struct Lift<T: Resource> {
    inner: Service<T, LiftReqBody>,
}

struct LiftBody<T: Resource> {
    body: ResponseBody<T>,
}

pub struct LiftReqBody {
    body: Body,
}

impl<T> Lift<T>
where
    T: Resource,
{
    fn new(inner: Service<T, LiftReqBody>) -> Self {
        Lift { inner }
    }
}

impl<T> Payload for LiftBody<T>
where
    T: Resource,
    T::Buf: Send,
{
    type Data = T::Buf;
    type Error = ::Error;

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, Self::Error> {
        self.body.poll()
    }
}

impl Stream for LiftReqBody {
    type Item = Chunk;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, ::Error> {
        self.body.poll()
            .map_err(|_| ::Error::Internal)
    }
}

impl<T> HyperService for Lift<T>
where
    T: Resource,
    T::Buf: Send,
{
    type ReqBody = Body;
    type ResBody = LiftBody<T>;
    type Error = ::Error;
    type Future = Box<Future<Item = http::Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, request: http::Request<Self::ReqBody>) -> Self::Future {
        use tower_service::Service;

        let request = request.map(|body| LiftReqBody { body });
        let response = self.inner.call(request)
            .map(|response| {
                response.map(|body| LiftBody { body })
            });

        Box::new(response)
    }
}

/// Run a service
pub fn run<T>(addr: &SocketAddr, service: Service<T, LiftReqBody>) -> io::Result<()>
where
    T: Resource,
    T::Buf: Send,
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
