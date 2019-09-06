mod attr;
mod extract;
mod response;
#[cfg(test)]
mod test;

use self::attr::Attribute;
use self::extract::Extract;
use self::response::Response;

use proc_macro2::TokenStream;
use syn::DeriveInput;

pub fn expand_derive_extract(input: DeriveInput) -> Result<TokenStream, String> {
    Extract::from_ast(input)
        .and_then(|extract| extract.gen())
}

pub fn expand_derive_response(input: DeriveInput) -> Result<TokenStream, String> {
    Response::from_ast(input)
        .and_then(|response| response.gen())
}
