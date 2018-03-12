#[macro_use]
extern crate proc_macro_hack;
extern crate syn;
// #[macro_use]
extern crate quote;

use quote::ToTokens;

proc_macro_item_impl! {
    /// Implement a Web Service
    pub fn impl_web_impl(input: &str) -> String {
        // Load the AST defining the web service
        let ast = syn::parse_str(input).unwrap();

        // AST transformer
        let mut v = MyFold;

        // Transfer the definition
        let ast = syn::fold::fold_file(&mut v, ast);

        /*
        let expanded = quote! {
            #ast
        };

        expanded.into()
        */
        ast.into_tokens().to_string()
    }
}

struct MyFold;

impl syn::fold::Fold for MyFold {
    fn fold_impl_item_method(&mut self, mut i: syn::ImplItemMethod) -> syn::ImplItemMethod {
        println!("ATTRS = {:?}", i.attrs);
        i.attrs.clear();
        i
    }
}

/*
#[proc_macro_derive(TowerWeb)]
pub fn derive_tower_web(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_str(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    gen.into()
}

/// Extract the `impl_web` source from the Rust macro hack enum.
fn extract_src(item: &syn::Item) -> Result<syn::File, Error> {
    use syn::{Expr, Item};

    let macro_hack_enum = match *item {
        Item::Enum(ref v) => v,
        _ => {
            panic!("not an enum; item = {:?}", item);
        }
    };

    // Make sure the enum is structured as expected
    assert_eq!(macro_hack_enum.ident, "RustMacroHack");
    assert_eq!(macro_hack_enum.variants.len(), 1);

    // Get the variant
    let macro_hack_variant = &macro_hack_enum.variants[0];
    assert_eq!(macro_hack_variant.ident, "Item");

    // The next step is to extract the source from the descriminant.
    let field = match macro_hack_variant.discriminant {
        Some((_, Expr::Field(ref field))) => field,
        _ => panic!("invalid"),
    };

    let tuple = match *field.base {
        Expr::Tuple(ref v) => v,
        _ => panic!(),
    };

    assert_eq!(tuple.elems.len(), 2);
    let expr = &tuple.elems[0];

    let tokens = match *expr {
        Expr::Macro(ref v) => v.mac.tts.clone(),
        _ => panic!(),
    };

    Ok(syn::parse2(tokens).unwrap())
}
*/
