//! Render content using templates
//!
//! Current, Handlebars is the only supported template engine. In time, the
//! API will be opened up to third party crates.

mod handlebars;

pub use self::handlebars::Handlebars;
