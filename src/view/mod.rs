//! Render content using templates
//!
//! Currently, Handlebars is the only supported template engine. In time, the
//! API will be opened up to third party crates.

mod handlebars;

pub use self::handlebars::Handlebars;
