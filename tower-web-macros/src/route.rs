use proc_macro2::{TokenStream, TokenTree};
use quote::TokenStreamExt;
use syn;

use std::fmt;

/// Represents a service route
pub struct Route {
    pub index: usize,

    /// Function identifier
    pub ident: syn::Ident,

    /// Function return type
    pub ret: syn::Type,

    pub rules: Rules,
}

#[derive(Debug)]
pub struct Rules {
    pub method: Option<Method>,

    /// HTTP path
    pub path: Option<String>,
    pub path_lit: Option<syn::LitStr>,
}

#[derive(Debug)]
pub enum Method {
    Get,
}

impl Route {
    pub fn new(index: usize, ident: syn::Ident, ret: syn::Type, rules: Rules) -> Self {
        Route {
            index,
            ident,
            ret,
            rules,
        }
    }

    pub fn destination_sym(&self) -> TokenStream {
        match self.index {
            0 => quote! { A(()) },
            1 => quote! { B(()) },
            2 => quote! { C(()) },
            3 => quote! { D(()) },
            4 => quote! { E(()) },
            5 => quote! { F(()) },
            6 => quote! { G(()) },
            7 => quote! { H(()) },
            8 => quote! { I(()) },
            9 => quote! { J(()) },
            _ => panic!("unimplemented; destination_sym"),
        }
    }
}

impl fmt::Debug for Route {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use quote::ToTokens;

        // TODO: Avoid escaping
        let ret = self.ret.clone().into_token_stream().to_string();

        fmt.debug_struct("Route")
            .field("ident", &self.ident.to_string())
            .field("ret", &ret)
            .field("rules", &self.rules)
            .finish()
    }
}

impl Rules {
    pub fn new() -> Rules {
        Rules {
            method: None,
            path: None,
            path_lit: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.method.is_none()
    }

    fn set_method(&mut self, value: Method) {
        assert!(self.method.is_none(), "unimplemented: dup method");
        self.method = Some(value);
    }

    /// Returns `true` if the attribute is processed
    pub fn process_attr(&mut self, attr: &syn::Attribute) -> bool {
        use syn::{Meta, Lit};

        let meta = match attr.interpret_meta() {
            Some(Meta::NameValue(meta)) => meta,
            _ => return false,
        };

        if meta.ident != "doc" {
            return false;
        }

        match meta.lit {
            Lit::Str(ref lit) => {
                // Extract the contents of the string literal
                let lit = lit.value();
                let raw = match trim_at_prefix(&lit) {
                    Some(raw) => raw,
                    None => return false,
                };

                self.process_doc_rule(&raw);

                true
            }
            _ => false,
        }
    }

    fn process_doc_rule(&mut self, doc: &str) {
        use syn::buffer;

        // Wrap the doc rule (with @ extracted) in an attribute.
        let mut attr = "#[".to_string();
        attr.push_str(doc);
        attr.push_str("]");

        // Convert that to a token stream
        let tokens: TokenStream = attr.parse().unwrap();

        // Get a TokenBuffer cursor
        let buffer = buffer::TokenBuffer::new2(tokens);
        let cursor = buffer.begin();

        // Parse the attribute
        let (attr, _) = syn::Attribute::parse_outer(cursor).unwrap();

        self.process_attr2(&attr);
    }

    /// Returns `true` if the attribute is processed
    fn process_attr2(&mut self, attr: &syn::Attribute) {
        use syn::Meta;

        match attr.interpret_meta() {
            Some(Meta::List(list)) => {
                assert!(list.nested.len() == 1, "unimplemeneted: invalid route rule");

                if list.ident == "GET" {
                    self.set_method(Method::Get);
                    self.process_path(&list);
                }
                else {
                    unimplemented!("unimplemeneted: invalid route rule");
                }
            }
            Some(_) => unimplemented!("unimplemeneted: invalid route rule"),
            None => unimplemented!("unimplemeneted: invalid route rule"),
        }
    }

    fn process_path(&mut self, list: &syn::MetaList) {
        use syn::{NestedMeta, Lit};

        assert!(list.nested.len() == 1, "unimplemeneted: invalid route rule");
        assert!(self.path.is_none(), "unimplemented: dup path");

        match list.nested.first().unwrap().value() {
            NestedMeta::Literal(Lit::Str(lit)) => {
                self.path = Some(lit.value());
                self.path_lit = Some(lit.clone());
            }
            _ => unimplemented!("unimplemented: invalid route rule"),
        }
    }
}

fn trim_at_prefix(s: &str) -> Option<&str> {
    for (i, b) in s.as_bytes().into_iter().enumerate() {
        match b {
            b' ' => {}
            b'@' => {
                return Some(&s[(i+1)..]);
            }
            _ => return None,
        }
    }

    None
}

// ===== impl Method =====

impl Method {
    pub fn to_tokens(&self) -> TokenStream {
        use self::Method::*;

        match *self {
            Get => quote! {
                ::tower_web::codegen::http::Method::GET
            },
        }
    }
}
