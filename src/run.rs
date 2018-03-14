use futures::stream::{self, Once};

use http;
use hyper;
use hyper::server::{Http, Service as HyperService};

use tokio;
use tokio::net::TcpListener;
use tokio::prelude::*;

use tower::{Service, NewService};

use std::{fmt, io};
use std::net::SocketAddr;
use std::sync::Arc;

struct Lift<T> {
    inner: T,
}

impl<T> Lift<T>
where T: Service,
{
    fn new(inner: T) -> Self {
        Lift { inner }
    }
}

impl<T> HyperService for Lift<T>
where T: Service<Request = http::Request<String>,
                Response = http::Response<String>> + Clone + Send + 'static,
      T::Future: Send,
{
    type Request = hyper::Request;
    type Response = hyper::Response<Once<String, hyper::Error>>;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error> + Send>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let req: http::Request<_> = req.into();
        let (head, body) = req.into_parts();

        let mut inner = self.inner.clone();

        let fut = body
            .concat2()
            .and_then(move |body| {
                // Convert the body to a string
                let body = String::from_utf8(body.to_vec()).unwrap();

                // Rebuild the request
                let req = http::Request::from_parts(head, body);

                // Call the inner service
                inner.call(req)
                    .map_err(|_| unimplemented!())
            })
            .map(|response| {
                response
                    .map(|body| stream::once(Ok(body)))
                    .into()
            });

        Box::new(fut)
    }
}

/// Run a service
pub fn run<T>(addr: &SocketAddr, new_service: T) -> io::Result<()>
where T: NewService<Request = http::Request<String>,
                   Response = http::Response<String>> + Send + 'static,
      T::InitError: fmt::Debug,
      T::Service: Clone + Send + 'static,
      T::Future: Send,
      <T::Service as Service>::Future: Send,
{
    let listener = TcpListener::bind(addr)?;
    let http = Arc::new(Http::<String>::new());

    tokio::run({
        listener
            .incoming()
            .map_err(|e| println!("failed to accept socket; err = {:?}", e))
            .for_each(move |socket| {
                let h = http.clone();

                tokio::spawn({
                    new_service.new_service()
                        .map_err(|e| println!("failed to build HTTP service; err = {:?}", e))
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
    });

    Ok(())
}
