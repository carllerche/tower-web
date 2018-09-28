#![doc(hidden)]

mod callsite;
pub use self::callsite::CallSite;
pub(crate) use self::callsite::Source;

pub mod bytes {
    //! Types provided by the `bytes` crate

    pub use bytes::*;
}

pub mod tower {
    //! Types provided by the `tower` crate

    pub use tower_service::Service;
}

pub mod http {
    //! Types provided by the `http` crate.
    pub use http::*;
}

pub mod futures {
    //! Types provided by the `futures` crate.
    pub use futures::*;
}

pub mod serde {
    pub use serde::*;
}

#[cfg(feature = "async-await-preview")]
pub mod async_await {
    use futures::Future;

    use std::future::{Future as StdFuture};

    /// Converts a `std::future::Future` to a boxed stable future.
    ///
    /// This bridges async/await with stable futures.
    pub fn async_to_box_future_send<T>(future: T)
        -> Box<Future<Item = T::Output, Error = ::Error> + Send>
    where
        T: StdFuture + Send + 'static,
    {
        use tokio_async_await::compat::backward;

        let future = backward::Compat::new(map_ok(future));
        Box::new(future)
    }

    async fn map_ok<T, E>(future: T) -> Result<T::Output, E>
    where
        T: StdFuture,
    {
        Ok(await!(future))
    }
}
