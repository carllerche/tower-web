use http::StatusCode;
use syn;
use proc_macro2::TokenStream;
use quote::ToTokens;

use std::collections::HashSet;

#[derive(Debug)]
pub(crate) struct Attributes {
    method: Option<Method>,

    /// HTTP path
    pub path: Option<String>,

    /// String literal version of the path
    pub path_lit: Option<syn::LitStr>,

    /// Path parameters
    pub path_params: Vec<String>,

    /// Produced content-type
    pub content_type: Option<String>,

    /// Catch
    catch: Option<Catch>,
}

#[derive(Debug)]
enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Debug)]
enum Catch {
    All,
    Status(HashSet<StatusCode>),
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {
            method: None,
            path: None,
            path_lit: None,
            path_params: vec![],
            content_type: None,
            catch: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.is_route() && !self.is_catch()
    }

    /// Returns true if the method is a route handler
    pub fn is_route(&self) -> bool {
        self.method.is_some()
    }

    /// Returns true if the method is a catch handler
    pub fn is_catch(&self) -> bool {
        self.catch.is_some()
    }

    fn set_method(&mut self, value: Method) {
        assert!(self.method.is_none(), "unimplemented: dup method");
        self.method = Some(value);
    }

    pub fn method_expr(&self) -> TokenStream {
        self.method.as_ref().unwrap().to_tokens()
    }

    pub fn path_expr(&self) -> TokenStream {
        self.path_lit.as_ref().unwrap().into_token_stream()
    }

    /// Returns `true` if the attribute is processed
    pub fn process(&mut self, attr: &syn::Attribute) -> bool {
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

                if self.method.is_some() && self.catch.is_some() {
                    panic!("catch handlers can not be routable");
                }

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
                assert!(list.nested.len() == 1, "unimplemented: invalid route rule; list.nested.len() == 1");

                // TODO: Should the identifier be lower cased?

                if list.ident == "get" {
                    self.set_method(Method::Get);
                    self.process_path(&list);
                } else if list.ident == "post" {
                    self.set_method(Method::Post);
                    self.process_path(&list);
                } else if list.ident == "put" {
                    self.set_method(Method::Put);
                    self.process_path(&list);
                } else if list.ident == "patch" {
                    self.set_method(Method::Patch);
                    self.process_path(&list);
                } else if list.ident == "delete" {
                    self.set_method(Method::Delete);
                    self.process_path(&list);
                } else if list.ident == "content_type" {
                    self.process_content_type(&list);
                } else if list.ident == "catch" {
                    self.process_catch(&list);
                } else {
                    println!("LIST; {:?}", list);
                    unimplemented!("unimplemented: invalid route rule");
                }
            }
            Some(Meta::Word(word)) => {
                if word == "catch" {
                    self.process_catch_all();
                } else {
                    println!("WORD; {:?}", word);
                    unimplemented!("unimplemented: invalid route rule");
                }
            }
            Some(meta) => unimplemented!("unimplemented: invalid route rule; META = {:?}", meta),
            None => unimplemented!("unimplemented: invalid route rule; Invalid meta"),
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
                    .filter(|segment| {
                        let c = segment.chars().next();
                        c == Some(':') || c == Some('*')
                    })
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

    fn process_catch_all(&mut self) {
        assert!(self.catch.is_none());
        self.catch = Some(Catch::All);
    }

    fn process_catch(&mut self, _list: &syn::MetaList) {
        unimplemented!("`@catch` must not have any additional attributes");
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

// ===== impl Method =====

impl Method {
    pub fn to_tokens(&self) -> TokenStream {
        use self::Method::*;

        match *self {
            Get => quote! { ::tower_web::codegen::http::Method::GET },
            Post => quote! { ::tower_web::codegen::http::Method::POST },
            Put => quote! { ::tower_web::codegen::http::Method::PUT },
            Patch => quote! { ::tower_web::codegen::http::Method::PATCH },
            Delete => quote! { ::tower_web::codegen::http::Method::DELETE },
        }
    }
}
