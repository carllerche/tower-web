#![recursion_limit = "512"]

#[macro_use]
extern crate proc_macro_hack;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

mod arg;
mod attr;
mod gen;
mod parse;
mod resource;
mod route;
mod ty_tree;

use arg::Arg;
use parse::*;
use resource::*;
use route::*;

const MAX_VARIANTS: usize = 3;

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
