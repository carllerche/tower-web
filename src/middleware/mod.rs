//! Middleware traits and implementations.
//!
//! A middleware decorates an service and provides additional functionality.
//! This additional functionality may include, but is not limited to:
//!
//! * Rejecting the request.
//! * Taking an action based on the request.
//! * Mutating the request before passing it along to the application.
//! * Mutating the response returned by the application.
//!
//! A middleware implements the [`Middleware`] trait.
//!
//! Currently, the following middleware implementations are provided:
//!
//! * [access logging][log]
//!
//! More will come.
//!
//! **Note**: This module will eventually be extracted out of `tower-web` into
//! `tower` and `tower-http`.
//!
//! [`Middleware`]: trait.Middleware.html
//! [log]: log/index.html

pub mod cors;
pub mod log;

mod chain;
mod identity;
mod middleware;

pub use self::chain::Chain;
pub use self::identity::Identity;
pub use self::middleware::Middleware;
