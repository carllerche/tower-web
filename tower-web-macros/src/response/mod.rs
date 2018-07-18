mod attr;
mod response;
#[cfg(test)]
mod test;

use self::attr::Attribute;
use self::response::Response;

use proc_macro2::TokenStream;
use syn::DeriveInput;

pub fn expand_derive_response(input: DeriveInput) -> Result<TokenStream, String> {
    Response::from_ast(input)
        .and_then(|response| response.gen())
}
