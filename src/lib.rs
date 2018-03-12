#[macro_use]
extern crate proc_macro_hack;

// This is what allows the users to depend on just your
// declaration crate rather than both crates.
#[allow(unused_imports)]
#[macro_use]
extern crate tower_web_derive;

#[doc(hidden)]
pub use tower_web_derive::*;

proc_macro_item_decl! {
    /// Implement a Web Service.
    impl_web! => impl_web_impl
}

