#![recursion_limit = "256"]
#![allow(warnings)]

#[macro_use]
extern crate proc_macro_hack;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

mod gen;
mod parse;
mod route;
mod service;

use parse::*;
use route::*;
use service::*;

proc_macro_item_impl! {
    /// Implement a Web Service
    pub fn impl_web_impl(input: &str) -> String {
        Parse::parse(input)
            .generate()
    }
}
