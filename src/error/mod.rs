//! Error types and traits.

mod catch;
mod error;
mod map;
mod never;

pub use self::catch::{Catch, IntoCatch, DefaultCatch, FnCatch};
pub use self::error::{Error, Builder, ErrorKind};
pub use self::map::Map;
pub(crate) use self::never::Never;
