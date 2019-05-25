use super::Attribute;

use syn::{self, DeriveInput};
use proc_macro2::{TokenStream, Span};
use quote::quote;

pub(crate) struct Extract {
    /// The response type identifier
    ty: syn::Ident,

    vis: syn::Visibility,

    /// Data (struct / enum) definition to interface with `serde`
    shadow_ty: DeriveInput,
}

impl Extract {
    pub fn from_ast(input: DeriveInput) -> Result<Extract, String> {
        use syn::fold::fold_derive_input;

        // The type of the data having `Response` derived
        let ty = input.ident.clone();
        let vis = input.vis.clone();

        // The hidden struct that is used to implement `serde::Deserialize`
        let shadow_ty = syn::Ident::new(
            &format!("Shadow{}", ty),
            Span::call_site());

        let mut fold_shadow_ty = FoldShadowTy {
            err: None,
        };

        // Fold thee shadow typee
        let mut output = fold_derive_input(&mut fold_shadow_ty, input);
        output.attrs.retain(is_serde_attr);

        if let Some(err) = fold_shadow_ty.err {
            return Err(err);
        }

        output.ident = shadow_ty;
        output.attrs.retain(|attr| !Attribute::is_web_attribute(attr));

        Ok(Extract {
            ty,
            vis,
            shadow_ty: output,
        })
    }

    pub fn gen(&self) -> Result<TokenStream, String> {
        let dummy_const = self.dummy_const();
        let ty = &self.ty;
        let vis = &self.vis;
        let shadow_ty = &self.shadow_ty.ident;
        let shadow_def = self.shadow_def();
        let from_shadow = self.from_shadow();

        Ok(quote! {
            #[allow(unused_variables, non_upper_case_globals)]
            const #dummy_const: () = {
                use tower_web as __tw;

                #shadow_def

                impl<B: __tw::util::BufStream> __tw::extract::Extract<B> for #ty {
                    type Future = ExtractFuture<B>;

                    fn extract(context: &__tw::extract::Context) -> Self::Future {
                        let inner = __tw::extract::serde::SerdeFuture::<_, B>::new_extract(context);
                        ExtractFuture { inner }
                    }

                    fn extract_body(context: &__tw::extract::Context, body: B) -> Self::Future {
                        let inner = __tw::extract::serde::SerdeFuture::new_extract_body(context, body);
                        ExtractFuture { inner }
                    }

                    fn requires_body(callsite: &__tw::codegen::CallSite) -> bool {
                        __tw::extract::serde::requires_body(callsite)
                    }
                }

                // Extract a value from the request.
                //
                // The implementation supports both synchronous extraction from
                // the request head as well as asynchronous extraction from the
                // request body.
                //
                // Because the extract type may have fields that are not handled
                // by serde, a shadow type must be used.
                #vis struct ExtractFuture<B> {
                    inner: __tw::extract::serde::SerdeFuture<#shadow_ty, B>,

                    // TODO: Store any other data required to extract the field
                }

                impl<B> __tw::extract::ExtractFuture for ExtractFuture<B>
                where B: __tw::util::BufStream,
                {
                    type Item = #ty;

                    fn poll(&mut self) -> __tw::codegen::futures::Poll<(), __tw::extract::Error> {
                        self.inner.poll()
                    }

                    fn extract(self) -> Self::Item {
                        let shadow = self.inner.extract();
                        #from_shadow
                    }
                }
            };
        })
    }

    fn shadow_def(&self) -> TokenStream {
        let shadow_data = &self.shadow_ty;

        quote! {
            #[derive(Deserialize)]
            #shadow_data
        }
    }

    fn dummy_const(&self) -> syn::Ident {
        syn::Ident::new(
            &format!("__IMPL_EXTRACT_FOR_{}", self.ty),
            Span::call_site())
    }

    fn from_shadow(&self) -> TokenStream {
        use syn::Data;

        match self.shadow_ty.data {
            Data::Struct(ref data_struct) => {
                from_shadow_struct(&self.ty, data_struct)
            }
            _ => unimplemented!(),
        }
    }
}

fn from_shadow_struct(
    ty: &syn::Ident,
    data_struct: &syn::DataStruct) -> TokenStream
{
    use syn::Fields;

    match data_struct.fields {
        Fields::Named(ref fields) => {
            let idents: Vec<_> = fields.named.iter()
                .map(|f| f.ident.as_ref().unwrap())
                .collect();

            let fields_1 = &idents;
            let fields_2 = &idents;

            quote! {
                #ty {
                    #(#fields_1: shadow.#fields_2,)*
                }
            }
        }
        Fields::Unnamed(ref fields) => {
            let fields = fields.unnamed.iter().enumerate()
                .map(|(i, _)| {
                    syn::Index::from(i)
                });

            quote! {
                #ty(
                    #(shadow.#fields,)*
                )
            }
        }
        Fields::Unit => unimplemented!(),
    }
}

struct FoldShadowTy {
    /// Any error encountered
    err: Option<String>,
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

        for mut field in named {
            assert!(field.ident.is_some(), "unimplemented: named fields with no name");

            let attrs = r#try!(Attribute::from_ast(&field.attrs));

            if attrs.is_empty() {
                field.attrs.retain(is_serde_attr);
                fields.named.push(field);
            } else {
                unimplemented!();
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

        // If an error has previously been encountered, do not do any work.
        if self.err.is_some() {
            return fields;
        }

        let unnamed = mem::replace(&mut fields.unnamed, Punctuated::new());

        for mut field in unnamed {
            assert!(field.ident.is_none(), "unimplemented: unnamed fields with name");

            let attrs = r#try!(Attribute::from_ast(&field.attrs));

            if attrs.is_empty() {
                field.attrs.retain(is_serde_attr);
                fields.unnamed.push(field);
            } else {
                unimplemented!();
            }
        }

        fields
    }
}

fn is_serde_attr(attr: &syn::Attribute) -> bool {
    use syn::Meta::*;

    attr.interpret_meta()
        .map(|meta| {
            match meta {
                List(ref list) => {
                    list.ident == "serde"
                }
                _ => false,
            }
        })
        .unwrap_or(false)
}
