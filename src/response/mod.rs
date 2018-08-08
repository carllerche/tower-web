//! Types and traits for responding to HTTP requests.
//!
//! The [`Response`] trait is implemented by types that can be converted to an
//! HTTP response. Resource methods must return futures that yield types
//! implementing [`Response`].
//!
//! Currently, [`Response`] implementations are provided for the following
//! types:
//!
//! * [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
//! * [`&'static str`](https://doc.rust-lang.org/std/primitive.str.html)
//! * [`http::Response`](https://docs.rs/http/0.1http/response/struct.Response.html)
//! * [`serde_json::Value`](https://docs.rs/serde_json/1/serde_json/enum.Value.html)
//! * [`tokio::fs::File`](https://docs.rs/tokio-fs/0.1/tokio_fs/file/struct.File.html)
//!
//! More implementations can be added by submitting a PR.
//!
//! Also, [`Response`] can be implemented for custom types by using the
//! [`derive(Response)`] proc macro. See [library level][lib] documentation for
//! more details.
//!
//! [`Response`]: trait.Response.html
//! [lib]: ../index.html

mod content_type;
mod context;
mod default_serializer;
mod file;
mod json;
mod response;
mod serde;
mod serializer;
mod str;

pub use self::content_type::ContentType;
pub use self::context::Context;
pub use self::default_serializer::DefaultSerializer;
pub use self::response::Response;
pub use self::serde::SerdeResponse;
pub use self::serializer::Serializer;
