mod context;
mod error;
mod immediate;
mod num;
/*
mod option;
mod str;
*/

pub use self::error::Error;
pub use self::context::Context;
pub use self::immediate::Immediate;

use futures::Poll;

/// Extract a value from a request.
pub trait Extract: Sized {
    type Future: ExtractFuture<Item = Self>;

    fn into_future(context: &Context) -> Self::Future;
}

/// Future representing the completion of extracting a value from a request
pub trait ExtractFuture {
    type Item;

    fn poll(&mut self) -> Poll<(), Error>;

    fn extract(self) -> Self::Item;
}
