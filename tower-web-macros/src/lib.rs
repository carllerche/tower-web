#![recursion_limit = "256"]

#[macro_use]
extern crate proc_macro_hack;
extern crate syn;
#[macro_use]
extern crate quote;

mod ast;
mod route;
mod service;

use route::*;
use service::*;

proc_macro_item_impl! {
    /// Implement a Web Service
    pub fn impl_web_impl(input: &str) -> String {
        ast::rewrite(input)
    }
}
