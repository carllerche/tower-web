#![recursion_limit = "256"]

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
    pub fn derive_resource_impl(input: &str) -> String {
        let gen = Parse::parse(input)
            .generate();

        println!("~~~~~~~~~~~~ GEN ~~~~~~~~~~~~~");
        println!("{}", gen);
        println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

        gen
    }
}
