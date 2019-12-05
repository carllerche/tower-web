#![recursion_limit = "512"]
#[deny(rust_2018_idioms)]

extern crate proc_macro;
use proc_macro::TokenStream;

use proc_macro2;
use syn;
use quote::quote;

mod derive;
mod header;
mod resource;

const MAX_VARIANTS: usize = 12;

/// Implement a Web Service
#[proc_macro]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    // Parse the input to a proc_macro2 token stream
    let input = syn::parse_macro_input!(input);

    // Generate the output
    resource::expand_derive_resource(input)
        // Convert it back to a proc_macro token stream
        .into()
}

#[proc_macro_derive(Extract, attributes(web, serde))]
pub fn derive_extract(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input);

    derive::expand_derive_extract(input)
        .unwrap_or_else(compile_error)
        .into()
}

#[proc_macro_derive(Response, attributes(web))]
pub fn derive_response(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input);

    derive::expand_derive_response(input)
        .unwrap_or_else(compile_error)
        .into()
}

fn compile_error(message: String) -> proc_macro2::TokenStream {
    quote! {
        compile_error!(#message);
    }
}
