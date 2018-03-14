//! Types re-exported by the library for use in codegen

pub mod tower {
    //! Types provided by the `tower` crate

    pub use ::tower::Service;
}

pub mod http {
    //! Types provided by the `http` crate.
    pub use ::http::{Request, Response};
}

pub mod futures {
    //! Types provided by the `futures` crate

    pub use ::futures::{Future, IntoFuture, Poll, Async};
}
