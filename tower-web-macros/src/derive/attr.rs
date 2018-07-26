use http::StatusCode;
use http::header::{HeaderName, HeaderValue};
use syn;

#[derive(Debug)]
pub(crate) struct Attribute {
    pub kind: Kind,
    pub source: syn::Attribute,
}

#[derive(Debug)]
pub(crate) enum Kind {
    Status(Option<StatusCode>),
    Header {
        name: Option<HeaderName>,
        value: Option<HeaderValue>,
    }
}

impl Attribute {
    pub(crate) fn is_web_attribute(attr: &syn::Attribute) -> bool {
        attr.path.segments.len() == 1 && attr.path.segments[0].ident == "web"
    }

    pub(crate) fn from_ast(attrs: &[syn::Attribute])
        -> Result<Vec<Attribute>, String>
    {
        use syn::{Meta, NestedMeta};

        let mut ret = vec![];

        for attr in attrs {
            if !Attribute::is_web_attribute(attr) {
                continue;
            }

            let meta = match attr.interpret_meta() {
                Some(meta) => meta,
                None => continue,
            };

            let source = attr.clone();

            match meta {
                Meta::List(meta_list) => {
                    for meta in &meta_list.nested {
                        let meta = match meta {
                            NestedMeta::Meta(meta) => meta,
                            NestedMeta::Literal(_) => {
                                unimplemented!("unexpected attribute literal; file={}; line={}", file!(), line!())
                            }
                        };

                        let attr = match meta {
                            Meta::Word(meta) => {
                                if meta == "header" {
                                    Attribute::header_from_word(&source)
                                } else if meta == "status" {
                                    Attribute::status_from_word(&source)
                                } else {
                                    unimplemented!("error handling");
                                }
                            }
                            Meta::List(meta) => {
                                if meta.ident == "header" {
                                    Attribute::header_from_list(meta, &source)
                                } else {
                                    let actual = quote!(#meta);

                                    return Err(format!("invalid struct level `status` annotation. The attribute must be in one of \
                                                        the following formats:\n\n\

                                                       `#[web(status)]`\n\
                                                       `#[web(status = \"201\")]`\n\n\

                                                       Actual: {}", actual.to_string()));
                                }
                            }
                            Meta::NameValue(meta) => {
                                if meta.ident == "status" {
                                    Attribute::status_from_name_value(meta, &source)
                                } else if meta.ident == "header" {
                                    unimplemented!("unexpected attribute; {:?}", meta);
                                } else {
                                    unimplemented!("unexpected attribute; {:?}", meta);
                                }
                            }
                        };

                        ret.push(attr);
                    }
                }
                _ => {
                    unimplemented!("file={}; line={}", file!(), line!());
                }
            }
        }

        Ok(ret)
    }

    fn status_from_word(source: &syn::Attribute) -> Attribute {
        Attribute {
            kind: Kind::Status(None),
            source: source.clone(),
        }
    }

    fn header_from_word(source: &syn::Attribute) -> Attribute {
        Attribute {
            kind: Kind::Header {
                name: None,
                value: None,
            },
            source: source.clone()
        }
    }

    fn status_from_name_value(
        meta: &syn::MetaNameValue,
        source: &syn::Attribute
    ) -> Attribute
    {
        use syn::Lit;

        let kind = match meta.lit {
            Lit::Str(ref lit_str) => {
                let lit_str = lit_str.value();
                let bytes = lit_str.as_bytes();
                let status = StatusCode::from_bytes(bytes)
                    .unwrap();

                Kind::Status(Some(status))
            }
            ref meta => unimplemented!("unsupported meta: {:?}", meta),
        };

        Attribute {
            kind,
            source: source.clone(),
        }
    }

    fn header_from_list(meta: &syn::MetaList, source: &syn::Attribute) -> Attribute {
        use syn::{NestedMeta, Meta, Lit};

        let mut name = None;
        let mut value = None;

        for meta in &meta.nested {
            match meta {
                NestedMeta::Meta(Meta::NameValue(meta)) => {
                    if meta.ident == "name" {
                        match meta.lit {
                            Lit::Str(ref v) => {
                                let hdr = v.value()
                                    .parse()
                                    .unwrap(); // TODO: Error handling

                                name = Some(hdr);
                            }
                            _ => unimplemented!("file={}; line={}", file!(), line!()),
                        }
                    } else if meta.ident == "value" {
                        match meta.lit {
                            Lit::Str(ref lit_str) => {
                                let lit_str = lit_str.value();
                                let bytes = lit_str.as_bytes();
                                let hdr_val = HeaderValue::from_bytes(bytes)
                                    .unwrap();

                                value = Some(hdr_val);
                            }
                            _ => unimplemented!("file={}; line={}", file!(), line!()),
                        }
                    } else {
                        unimplemented!("file={}; line={}", file!(), line!());
                    }
                }
                meta => unimplemented!("unsupported meta: {:?}", meta),
            }
        }

        Attribute {
            kind: Kind::Header { name, value },
            source: source.clone(),
        }
    }
}
