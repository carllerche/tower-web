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

/// Implement a Web Service
pub fn derive(input: &str) -> String {
    let gen = Parse::parse(input)
        .generate();

    println!("~~~~~~~~~~~~ GEN ~~~~~~~~~~~~~");
    println!("{}", gen);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    gen
}
