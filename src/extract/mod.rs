mod bytes;
mod context;
mod error;
mod immediate;
mod num;
pub mod option;
mod pathbuf;
pub mod serde;
mod str;
pub mod http_date_time;

pub use self::error::Error;
pub use self::context::Context;
pub use self::immediate::Immediate;

use codegen::CallSite;
use util::BufStream;

use futures::Poll;

/// Extract a value from a request.
///
/// The trait is generic over `B: BufStream`, which represents the HTTP request
/// body stream.
pub trait Extract<B: BufStream>: 'static + Sized {
    type Future: ExtractFuture<Item = Self>;

    /// Extract the argument
    fn extract(context: &Context) -> Self::Future;

    /// Extract the argument using the body
    fn extract_body(context: &Context, body: B) -> Self::Future {
        drop((context, body));
        panic!("The default implementation of `Extract::extract_body` should never be called")
    }

    /// Returns `true` if extracting the type requires `body`.
    fn requires_body(callsite: &CallSite) -> bool {
        drop(callsite);
        false
    }
}

/// Future representing the completion of extracting a value from a request
pub trait ExtractFuture {
    type Item;

    fn poll(&mut self) -> Poll<(), Error>;

    fn extract(self) -> Self::Item;
}
