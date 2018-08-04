mod arg;
mod attr;
mod parse;
mod resource;
mod route;
mod ty_tree;

use self::arg::Arg;
use self::attr::Attributes;
use self::parse::*;
use self::resource::*;
use self::route::*;
use self::ty_tree::TyTree;

use proc_macro2::TokenStream;

/// Implement a Web Service
pub fn expand_derive_resource(input: TokenStream) -> TokenStream {
    Parse::parse(input)
        .generate()
}
