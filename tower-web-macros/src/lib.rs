#![recursion_limit = "512"]

#[macro_use]
extern crate proc_macro_hack;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

mod resource;

const MAX_VARIANTS: usize = 3;

proc_macro_item_impl! {
    /// Implement a Web Service
    pub fn derive_resource_impl(input: &str) -> String {
        resource::derive(input)
    }
}
