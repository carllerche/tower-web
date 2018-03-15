//! Types re-exported by the library for use in codegen

use bytes::Bytes;
use futures::{Future, Stream};
use http::Response;

/// A boxed Resource response
pub type BoxResponse<T> = Box<Future<Item = Response<T>, Error = ::Error> + Send>;

/// A boxed streaming body
pub type BoxBody = Box<Stream<Item = Bytes, Error = ::Error> + Send>;


pub mod tower {
    //! Types provided by the `tower` crate

    pub use ::tower::Service;
}

pub mod http {
    //! Types provided by the `http` crate.
    pub use ::http::{Request, Response};
}

pub mod futures {
    //! Types provided by the `futures` crate.
    pub use ::futures::*;
}
