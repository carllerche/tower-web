#![recursion_limit = "512"]
#[deny(rust_2018_idioms)]

use proc_macro2;
use syn;
use proc_macro_hack::proc_macro_item_impl;
use quote::quote;

mod derive;
mod header;
mod resource;

use derive_resource_impl::TokenStream;

const MAX_VARIANTS: usize = 12;

proc_macro_item_impl! {
    /// Implement a Web Service
    pub fn derive_resource_impl(input: &str) -> String {
        // Parse the input to a token stream
        let input = syn::parse_str(input).unwrap();

        // Generate the output
        resource::expand_derive_resource(input)
            // Convert the TokenStream back to a string
            .to_string()
    }
}

#[proc_macro_derive(Extract, attributes(web, serde))]
pub fn derive_extract(input: TokenStream) -> TokenStream {
    // Parse the input to `DeriveInput`
    let input = syn::parse(input).unwrap();

    derive::expand_derive_extract(input)
        .unwrap_or_else(compile_error)
        .into()
}

#[proc_macro_derive(Response, attributes(web))]
pub fn derive_response(input: TokenStream) -> TokenStream {
    // Parse the input to `DeriveInput`
    let input = syn::parse(input).unwrap();

    derive::expand_derive_response(input)
        .unwrap_or_else(compile_error)
        .into()
}

fn compile_error(message: String) -> proc_macro2::TokenStream {
    quote! {
        compile_error!(#message);
    }
}
