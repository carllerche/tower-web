use proc_macro2::TokenStream;
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

    pub args: Vec<Arg>,
}

#[derive(Debug)]
pub struct Arg {
    /// Argument identifier, i.e., the variable name.
    pub ident: Option<String>,

    /// The index of the path binding the identifier matches.
    pub param: Option<usize>,

    // The argument type
    pub ty: syn::Type,
}

#[derive(Debug)]
pub struct Rules {
    pub method: Option<Method>,

    /// HTTP path
    pub path: Option<String>,

    /// String literal version of the path
    pub path_lit: Option<syn::LitStr>,

    /// Path parameters
    pub path_params: Vec<String>,

    /// Produced content-typee
    pub content_type: Option<String>,
}

#[derive(Debug)]
pub enum Method {
    Get,
}

impl Route {
    pub fn new(
        index: usize,
        ident: syn::Ident,
        ret: syn::Type,
        rules: Rules,
        args: Vec<Arg>,
    ) -> Self {
        Route {
            index,
            ident,
            ret,
            rules,
            args,
        }
    }

    pub fn destination_sym(&self, content: TokenStream) -> TokenStream {
        match self.index {
            0 => quote! { A(#content) },
            1 => quote! { B(#content) },
            2 => quote! { C(#content) },
            3 => quote! { D(#content) },
            4 => quote! { E(#content) },
            5 => quote! { F(#content) },
            6 => quote! { G(#content) },
            7 => quote! { H(#content) },
            8 => quote! { I(#content) },
            9 => quote! { J(#content) },
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
            path_params: vec![],
            content_type: None,
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
        use syn::{Lit, Meta};

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

                if list.ident == "get" {
                    self.set_method(Method::Get);
                    self.process_path(&list);
                } else if list.ident == "content_type" {
                    self.process_content_type(&list);
                } else {
                    println!("LIST; {:?}", list);
                    unimplemented!("unimplemeneted: invalid route rule");
                }
            }
            Some(_) => unimplemented!("unimplemeneted: invalid route rule"),
            None => unimplemented!("unimplemeneted: invalid route rule"),
        }
    }

    fn process_path(&mut self, list: &syn::MetaList) {
        use syn::{Lit, NestedMeta};

        assert!(list.nested.len() == 1, "unimplemeneted: invalid route rule");
        assert!(self.path.is_none(), "unimplemented: dup path");

        match list.nested.first().unwrap().value() {
            NestedMeta::Literal(Lit::Str(lit)) => {
                // Convert the path literal to a String
                let path = lit.value();

                // Figure out param indices
                //
                // TODO: Validate param format
                self.path_params = path.split("/")
                    .filter(|segment| segment.chars().next() == Some(':'))
                    .map(|segment| segment[1..].to_string())
                    .collect();

                self.path = Some(path);
                self.path_lit = Some(lit.clone());
            }
            _ => unimplemented!("unimplemented: invalid route rule"),
        }
    }

    fn process_content_type(&mut self, list: &syn::MetaList) {
        use syn::{Lit, NestedMeta};

        assert!(list.nested.len() == 1, "unimplemeneted: invalid route rule");
        assert!(self.content_type.is_none(), "content_type already set");

        match list.nested.first().unwrap().value() {
            NestedMeta::Literal(Lit::Str(lit)) => {
                self.content_type = Some(lit.value());
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
                return Some(&s[(i + 1)..]);
            }
            _ => return None,
        }
    }

    None
}

// ===== impl Arg =====

impl Arg {
    pub fn new(ident: String, param: Option<usize>, ty: syn::Type) -> Arg {
        Arg {
            ident: Some(ident),
            param,
            ty,
        }
    }

    pub fn ty_only(ty: syn::Type) -> Arg {
        Arg {
            ty,
            ident: None,
            param: None,
        }
    }
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
