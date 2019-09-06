//! Asynchronous stream of bytes.
//!
//! This module contains the `BufStream` trait and a number of combinators for
//! this trait. The trait is similar to `Stream` in the `futures` library, but
//! instead of yielding arbitrary values, it only yields types that implement
//! `Buf` (i.e, byte collections).
//!
//! Having a dedicated trait for this case enables greater functionality and
//! ease of use.
//!
//! This module will eventually be moved into Tokio.

mod buf_stream;
mod bytes;
mod chain;
mod collect;
pub mod deflate;
mod either;
mod empty;
mod file;
mod from;
pub mod size_hint;
mod std;
mod str;

pub use self::buf_stream::BufStream;
pub use self::chain::Chain;
pub use self::collect::Collect;
pub use self::empty::{empty, Empty};
pub use self::from::FromBufStream;
pub use self::size_hint::SizeHint;
pub use self::std::StdStream;
