mod arg;
mod attr;
mod catch;
mod parse;
mod signature;
mod resource;
mod route;
mod ty_tree;

use self::arg::Arg;
use self::attr::Attributes;
use self::catch::Catch;
use self::parse::*;
use self::signature::Signature;
use self::resource::*;
use self::route::*;
use self::ty_tree::TyTree;

use proc_macro2::TokenStream;

/// Implement a Web Service
pub fn expand_derive_resource(input: TokenStream) -> TokenStream {
    Parse::parse(input)
        .generate()
}
