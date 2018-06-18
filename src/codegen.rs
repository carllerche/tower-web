//! Types re-exported by the library for use in codegen

pub mod bytes {
    //! Types provided by the `bytes` crate

    pub use bytes::{Buf, Bytes};
}

pub mod tower {
    //! Types provided by the `tower` crate

    pub use tower_service::Service;
}

pub mod http {
    //! Types provided by the `http` crate.
    pub use http::{Method, Request, Response};
}

pub mod futures {
    //! Types provided by the `futures` crate.
    pub use futures::*;
}
