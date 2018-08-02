use super::HttpService;
use util::buf_stream::BufStream;

use futures::Future;
use http::{Request, Response};
use tower_service::NewService;

pub trait NewHttpService: sealed::Sealed {
    type RequestBody: BufStream;

    type ResponseBody: BufStream;

    type Error;

    type Service: HttpService<RequestBody = Self::RequestBody,
                             ResponseBody = Self::ResponseBody,
                                    Error = Self::Error>;

    type InitError;

    type Future: Future<Item = Self::Service, Error = Self::InitError>;

    fn new_service(&self) -> Self::Future;
}

impl<T, B1, B2> NewHttpService for T
where T: NewService<Request = Request<B1>,
                   Response = Response<B2>>,
      B1: BufStream,
      B2: BufStream
{
    type RequestBody = B1;
    type ResponseBody = B2;
    type Error = T::Error;
    type Service = T::Service;
    type InitError = T::InitError;
    type Future = T::Future;

    fn new_service(&self) -> Self::Future {
        NewService::new_service(self)
    }
}

impl<T, B1, B2> sealed::Sealed for T
where T: NewService<Request = Request<B1>,
                   Response = Response<B2>>,
      B1: BufStream,
      B2: BufStream
{}

mod sealed {
    pub trait Sealed {}
}
