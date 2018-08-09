//! Extract data from the HTTP request.
//!
//! The [`Extract`] trait is implemented by types that can be constructed by
//! using data from an HTTP request. Resource method argument types must
//! implement [`Extract`]. This is how `impl_web!` is able to instantiate them
//! using the request. See [library level][lib] documentation for more details.
//!
//! Currently, [`Extract`] implementations are provided for the following types:
//!
//! * [`Bytes`](https://docs.rs/bytes/0.4/bytes/struct.Bytes.html)
//! * [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
//! * [`PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html)
//! * [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
//! * [`Vec<u8>`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
//!
//! More implementations can be added by submitting a PR.
//!
//! Also, [`Extract`] can be implemented for custom types by using the
//! [`derive(Extract)`] proc macro. See [library level][lib] documentation for
//! more details.
//!
//! [`Extract`]: trait.Extract.html
//! [lib]: ../index.html

mod bytes;
mod context;
mod error;
mod immediate;
mod num;
pub mod option;
mod pathbuf;
#[doc(hidden)]
pub mod serde;
mod str;

pub use self::error::Error;
pub use self::context::Context;
pub use self::immediate::Immediate;

use codegen::CallSite;
use util::BufStream;

use futures::Poll;

/// Extract a value from an HTTP request.
///
/// The extracted value does not need to be produced immediately.
/// Implementations of `Extract` are able to perform asynchronous processing.
///
/// The trait is generic over `B: BufStream`, which represents the HTTP request
/// body stream.
pub trait Extract<B: BufStream>: 'static + Sized {
    /// The future representing the completion of the extraction logic.
    type Future: ExtractFuture<Item = Self>;

    /// Extract the argument from the HTTP request.
    ///
    /// This function is not provide the HTTP request body. Implementations of
    /// this function must ensure that the request HEAD (request URI and
    /// headers) are sufficient for extracting the value.
    fn extract(context: &Context) -> Self::Future;

    /// Extract the argument using the HTTP request body.
    ///
    /// Doing so will usually involve deserializing the contents of the HTTP
    /// request body to the target value being extracted.
    fn extract_body(context: &Context, body: B) -> Self::Future {
        drop((context, body));
        panic!("The default implementation of `Extract::extract_body` should never be called")
    }

    /// Returns `true` if extracting the type requires `body`.
    ///
    /// Only a single resource method argument may extract using the HTTP
    /// request body. This function allows enforcing this requirement.
    fn requires_body(callsite: &CallSite) -> bool {
        drop(callsite);
        false
    }
}

/// Future representing the completion of extracting a value from a request
///
/// Implementations are expected to advance to a state where `extract` will
/// succeed. This is done by performing any asynchronous processing when `poll`
/// is called and usually stashing the extracted value internally until
/// `extract` is called.
///
/// `extract` must not be called until `poll` returns `Ok(Ready(()))`.
pub trait ExtractFuture {
    /// The argument extracted from the request.
    type Item;

    /// Returns `Ok(Ready(()))` when `extract()` can be called. If `NotReady` is
    /// returned, the current task is registered for wakeup.
    fn poll(&mut self) -> Poll<(), Error>;

    /// Consume `self` and return the value extracted from the HTTP request.
    fn extract(self) -> Self::Item;
}
