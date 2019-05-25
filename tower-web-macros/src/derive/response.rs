use super::{attr, Attribute};

use http::{self, HeaderMap};
use http::header::HeaderName;
use proc_macro2::{TokenStream, Span};
use quote::ToTokens;
use syn::{self, Data, DeriveInput, Ident};
use syn::spanned::Spanned;
use quote::{quote, quote_spanned};

pub(crate) struct Response {
    /// The response type identifier
    ty: Ident,

    /// Any generics on the type
    generics: syn::Generics,

    /// How to obtain the status code for the response
    status: Option<StatusCode>,

    /// HTTP headers to add to the response
    static_headers: HeaderMap,

    /// HTTP headers to get from struct fields
    dyn_headers: Vec<HeaderField>,

    /// The template to use, if one is set
    template: Option<String>,

    /// Delegate the Response implementation to the enum fields
    either: bool,

    /// Data (struct / enum) definition to interface with `serde`
    shadow_ty: DeriveInput,
}

/// How to get the status code for a response
enum StatusCode {
    /// The status code is set for the struct
    Static(http::StatusCode),

    /// The status code is determined by a field
    Dynamic(Ident),
}

/// Struct field representing an HTTP header
struct HeaderField {
    ident: Ident,
    name: HeaderName,
}

impl Response {
    pub fn from_ast(input: DeriveInput) -> Result<Response, String> {
        use syn::fold::fold_derive_input;

        // The type of the data having `Response` derived
        let ty = input.ident.clone();

        // Any generics
        let generics = input.generics.clone();

        let mut status = None;
        let mut static_headers = HeaderMap::new();
        let mut either = false;
        let mut template = None;

        for attribute in Attribute::from_ast(&input.attrs)? {
            match attribute.kind {
                attr::Kind::Status(Some(value)) => {
                    // The response status code is static for the response type
                    status = Some(StatusCode::Static(value));
                }
                attr::Kind::Status(None) => {
                    return Err("invalid struct level `status` annotation. The annotation\
                                must include a value. For example: \n\n\
                               `#[web(status = \"201\")]`".to_string());
                }
                attr::Kind::Header { name, value } => {
                    let name = match name {
                        Some(n) => n,
                        None => unimplemented!("error handling"),
                    };

                    let value = match value {
                        Some(v) => v,
                        None => unimplemented!("error handling"),
                    };

                    static_headers.append(name, value);
                }
                attr::Kind::Template(value) => {
                    if template.is_some() {
                        return Err("struct must have at most one `template` annotation.".to_string());
                    }

                    template = Some(value);
                }
                attr::Kind::Either => {
                    either = true;
                }
            }
        }

        // The hidden struct that is used to implement `serde::Serialize`.
        let shadow_ty = Ident::new(
            &format!("Shadow{}", ty),
            Span::call_site());

        // Walks the `DeriveInput` syntax tree, observing and transforming.
        let mut fold_shadow_ty = FoldShadowTy {
            src_fields: None,
            status_field: None,
            header_fields: vec![],
            err: None,
        };

        // Fold the shadow type
        let mut output = fold_derive_input(&mut fold_shadow_ty, input);

        if let Some(err) = fold_shadow_ty.err {
            return Err(err);
        }

        if let Some(field) = fold_shadow_ty.status_field {
            if status.is_some() {
                return Err(format!("duplicate `status` annotation. There must only be \
                                    a single `status` annotation either at the struct \
                                    level or the field level. Both the struct and the field \
                                    named `{}` are annotated.", field));
            }

            status = Some(StatusCode::Dynamic(field));
        }

        // Modify the output to update the data name and remove `web` attrs
        output.ident = shadow_ty;
        output.attrs.retain(|attr| !Attribute::is_web_attribute(attr));

        let dyn_headers = fold_shadow_ty.header_fields;

        if either
            && (status.is_some() || !static_headers.is_empty() || !dyn_headers.is_empty())
        {
            return Err(String::from(
                "`#[web(either)]` cannot be used together with other `#[web]` attributes.",
            ));
        }

        Ok(Response {
            ty,
            generics,
            status,
            static_headers,
            dyn_headers,
            either,
            template,
            shadow_ty: output,
        })
    }

    pub fn gen(&self) -> Result<TokenStream, String> {
        if self.either {
            self.gen_enum()
        } else {
            self.gen_serde()
        }
    }

    pub fn gen_enum(&self) -> Result<TokenStream, String> {
        let dummy_const = self.dummy_const();
        let ty = &self.ty;

        let (body_type, cases) = match self.shadow_ty.data {
            Data::Enum(ref data) => {
                let either_type =
                    Ident::new(&format!("Either{}", data.variants.len()), Span::call_site());

                let args = data
                    .variants
                    .iter()
                    .map(|variant| match variant.fields {
                        syn::Fields::Unnamed(ref fields) => match fields.unnamed.len() {
                            1 => {
                                let ty = &fields.unnamed[0].ty;
                                let span = variant.ident.span();
                                Ok((
                                    quote_spanned!(span=> <#ty as __tw::response::Response>::Body),
                                    quote_spanned!(span=> #ty),
                                ))
                            }
                            _ => Err(String::from(
                                "only single-field variants are supported for `#[web(either)]`",
                            )),
                        },
                        _ => Err(String::from(
                            "only unnamed fields are supported for `#[web(either)]`",
                        )),
                    }).collect::<Result<Vec<_>, _>>()?;

                let (body_args, either_args): (Vec<_>, Vec<_>) = args.into_iter().unzip();

                let body_type = quote! { __tw::util::tuple::#either_type<#(#body_args),*> };
                let either_args = quote! { <#(#either_args),*> };

                let cases = {
                    let cases = data.variants.iter().enumerate().map(|(i, variant)| {
                        let case = &variant.ident;
                        let span = variant.span();
                        let ctor = Ident::new(&((i as u8 + b'A') as char).to_string(), span);
                        quote_spanned! {span=>
                            #ty::#case(inner) => __tw::util::tuple::#either_type::#ctor::#either_args(inner).into_http(context)?
                        }
                    });
                    quote! { #(#cases),* }
                };

                (body_type, cases)
            }
            _ => {
                return Err(String::from(
                    "only enums are supported for `#[web(either)]`",
                ))
            }
        };

        Ok(quote! {
            #[allow(unused_variables, non_upper_case_globals)]
            const #dummy_const: () = {
                use tower_web as __tw;

                impl __tw::response::Response for #ty {
                    type Buf = <Self::Body as __tw::util::BufStream>::Item;
                    type Body = #body_type;

                    fn into_http<S: __tw::response::Serializer>(
                        self,
                        context: &__tw::response::Context<S>,
                    ) -> Result<__tw::codegen::http::Response<Self::Body>, __tw::Error>
                    {
                        Ok(match self {
                            #cases
                        })
                    }
                }
            };
        })
    }

    pub fn gen_serde(&self) -> Result<TokenStream, String> {
        let dummy_const = self.dummy_const();
        let ty = &self.ty;
        let shadow_ty = &self.shadow_ty.ident;
        let shadow_def = self.shadow_def();
        let status = self.status();
        let static_headers = self.static_headers();
        let dyn_headers = self.dyn_headers();
        let template = self.template();
        let generics = self.generic_idents();

        let generics_serialize = self.generics_bound_by(
            quote!(__tw::codegen::serde::Serialize));

        let generics_lt = self.generics_bound_by(
            quote!('__a + __tw::codegen::serde::Serialize));

        Ok(quote! {
            #[allow(unused_variables, non_upper_case_globals)]
            const #dummy_const: () = {
                use tower_web as __tw;

                impl<#generics_serialize> __tw::response::Response for #ty<#generics>
                {
                    type Buf = <Self::Body as __tw::util::BufStream>::Item;
                    type Body = __tw::error::Map<__tw::codegen::bytes::Bytes>;

                    fn into_http<S: __tw::response::Serializer>(
                        self,
                        context: &__tw::response::Context<S>,
                    ) -> Result<__tw::codegen::http::Response<Self::Body>, __tw::Error>
                    {
                        struct Lift<'__a, #generics_lt>(&'__a #ty<#generics>);

                        impl<'__a, #generics_lt> __tw::codegen::serde::Serialize for Lift<'__a, #generics>
                        {
                            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                            where S: __tw::codegen::serde::Serializer
                            {
                                #shadow_ty::serialize(self.0, serializer)
                            }
                        }

                        #[allow(unused_mut)]
                        let mut serializer_context = context.serializer_context();
                        #template

                        // TODO: Improve and handle errors
                        let body: __tw::codegen::bytes::Bytes = context.serialize(&Lift(&self), &serializer_context)
                            .map_err(|_| __tw::Error::from(__tw::codegen::http::status::StatusCode::INTERNAL_SERVER_ERROR))?;

                        let body = __tw::error::Map::new(body);

                        let mut response = __tw::codegen::http::Response::builder()
                            // Customize response
                            .status(#status)
                            #(#static_headers)*
                            #(#dyn_headers)*
                            .body(body)
                            .unwrap();

                        response
                            .headers_mut()
                            .entry(__tw::codegen::http::header::CONTENT_TYPE)
                            .unwrap()
                            .or_insert_with(|| {
                                context.content_type_header()
                                    .cloned()
                                    .unwrap_or_else(|| {
                                        __tw::codegen::http::header::HeaderValue::from_static("application/octet-stream")
                                    })
                            });

                        Ok(response)
                    }
                }

                #shadow_def
            };
        })
    }

    fn status(&self) -> TokenStream {
        match self.status {
            Some(StatusCode::Static(ref s)) => {
                let status = s.as_str();
                quote!(#status)
            }
            Some(StatusCode::Dynamic(ref field)) => {
                quote!(self.#field)
            }
            None => quote!("200"),
        }
    }

    fn static_headers<'a>(&'a self) -> impl Iterator<Item = TokenStream> + 'a {
        self.static_headers.iter()
            .map(|(key, value)| {
                let key = key.as_str();
                // TODO: Don't go via string
                let value = value.to_str().unwrap();

                quote! {
                    .header(#key, #value)
                }
            })
    }

    fn dyn_headers<'a>(&'a self) -> impl Iterator<Item = TokenStream> + 'a {
        self.dyn_headers.iter()
            .map(|header_field| {
                let ident = &header_field.ident;
                let name = header_field.name.as_str();

                quote! {
                    .header(#name, self.#ident)
                }
            })
    }

    fn template(&self) -> TokenStream {
        match self.template {
            Some(ref template) => {
                quote!(serializer_context.set_template(#template);)
            }
            None => quote!(),
        }
    }

    fn shadow_def(&self) -> TokenStream {
        let ty = &self.ty.to_string();
        let shadow_data = &self.shadow_ty;

        quote! {
            #[derive(Serialize)]
            #[serde(remote = #ty)]
            #shadow_data
        }
    }

    fn generic_idents(&self) -> TokenStream {
        use syn::GenericParam::Type;

        let generics = self.generics.params.iter()
            .map(|param| {
                match *param {
                    Type(ref type_param) => {
                        &type_param.ident
                    }
                    _ => unimplemented!(),
                }
            });

        quote! {
            #(#generics),*
        }
    }

    fn generics_bound_by(&self, bound: TokenStream) -> TokenStream {
        use syn::GenericParam::Type;

        let generics = self.generics.params.iter()
            .map(|param| {
                match *param {
                    Type(ref type_param) => {
                        let ident = &type_param.ident;

                        if type_param.bounds.is_empty() {
                            quote! {
                                #ident: #bound
                            }
                        } else {
                            let existing = type_param.bounds.iter();

                            quote! {
                                #ident: #(#existing)+* + #bound
                            }
                        }
                    }
                    _ => unimplemented!(),
                }
            });

        quote! {
            #(#generics),*
        }
    }

    fn dummy_const(&self) -> Ident {
        Ident::new(
            &format!("__IMPL_RESPONSE_FOR_{}", self.ty),
            Span::call_site())
    }
}

impl Fields {
    /// # Panics
    ///
    /// Panics if `self` represents unnamed fields
    fn named(&mut self) -> &mut Vec<Ident> {
        match *self {
            Fields::Named(ref mut s) => s,
            _ => panic!(),
        }
    }

    /// # Panics
    ///
    /// Panics if `self` represents named fields
    fn unnamed(&mut self) -> &mut Vec<syn::Index> {
        match *self {
            Fields::Unnamed(ref mut s) => s,
            _ => panic!(),
        }
    }
}

struct FoldShadowTy {
    /// Fields in original structure to use when converting to the shadow
    /// structure.
    src_fields: Option<Fields>,

    /// Field representing the HTTP status code
    status_field: Option<Ident>,

    /// Fields representing HTTP headers
    header_fields: Vec<HeaderField>,

    /// Any error encountered
    err: Option<String>,
}

enum Fields {
    Named(Vec<Ident>),
    Unnamed(Vec<syn::Index>),
}

impl syn::fold::Fold for FoldShadowTy {
    fn fold_fields_named(&mut self, mut fields: syn::FieldsNamed) -> syn::FieldsNamed {
        use syn::punctuated::Punctuated;
        use std::mem;

        macro_rules! r#try {
            ($e:expr) => {{
                match $e {
                    Ok(ret) => ret,
                    Err(err) => {
                        self.err = Some(err);
                        return fields;
                    }
                }
            }}
        }

        // If an error has previously been encountered, do not do any work.
        if self.err.is_some() {
            return fields;
        }

        let named = mem::replace(&mut fields.named, Punctuated::new());

        for field in named {
            assert!(field.ident.is_some(), "unimplemented: fields with no name");

            let attrs = r#try!(Attribute::from_ast(&field.attrs));

            if attrs.is_empty() {
                self.src_fields
                    .get_or_insert_with(|| Fields::Named(vec![]))
                    .named()
                    .push(field.ident.clone().unwrap());

                fields.named.push(field);
            } else {
                for attr in attrs {
                    match attr.kind {
                        attr::Kind::Status(code) => {
                            if let Some(ref curr) = self.status_field {
                                let field = field.ident.unwrap();
                                self.err = Some(format!("duplicate field level `status` annotation. Only a single field \
                                                         may be annotated with `status`. However, both `{}` and `{}` have \
                                                         the annotation.", field, curr));
                                return fields;
                            }


                            if code.is_some() {
                                self.err = Some(format!("invalid field level `status` annotation. The annotation must \
                                                         not supply a value. The form must be:\n\n\

                                                         `#[web(status)]`\n\n\

                                                         Actual: {}", attr.source.into_token_stream().to_string()));
                                return fields;
                            }

                            self.status_field = field.ident.clone();
                        }
                        attr::Kind::Header { name, value } => {
                            assert!(value.is_none(), "unimplemented: handling value on header field");

                            let ident = field.ident.clone().unwrap();
                            let name = match name {
                                Some(name) => name,
                                None => {
                                    let arg = ident.to_string();
                                    crate::header::arg_to_header_name(&arg)
                                }
                            };

                            self.header_fields.push(HeaderField {
                                ident,
                                name,
                            });
                        }
                        attr::Kind::Template(_) => {
                            self.err = Some(format!("`template` attribute must be at the struct level."));
                            return fields;
                        }
                        attr::Kind::Either => {}
                    }
                }
            }
        }

        fields
    }

    fn fold_fields_unnamed(&mut self, mut fields: syn::FieldsUnnamed) -> syn::FieldsUnnamed {
        use syn::punctuated::Punctuated;
        use std::mem;

        macro_rules! r#try {
            ($e:expr) => {{
                match $e {
                    Ok(ret) => ret,
                    Err(err) => {
                        self.err = Some(err);
                        return fields;
                    }
                }
            }}
        }


        if self.err.is_some() {
            return fields;
        }

        let unnamed = mem::replace(&mut fields.unnamed, Punctuated::new());

        for (i, field) in unnamed.into_iter().enumerate() {
            assert!(field.ident.is_none(), "unimplemented: field with name");

            let attrs = r#try!(Attribute::from_ast(&field.attrs));

            if attrs.is_empty() {
                let index = syn::Index::from(i);

                self.src_fields
                    .get_or_insert_with(|| Fields::Unnamed(vec![]))
                    .unnamed()
                    .push(index);

                fields.unnamed.push(field);
            } else {
                unimplemented!("file={}; line={}", file!(), line!());
            }
        }

        fields
    }
}
