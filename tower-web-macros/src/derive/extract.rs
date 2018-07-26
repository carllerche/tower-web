use super::Attribute;

use syn::{self, DeriveInput};
use proc_macro2::{TokenStream, Span};

pub(crate) struct Extract {
    /// The response type identifier
    ty: syn::Ident,

    vis: syn::Visibility,

    /// Data (struct / enum) definition to interface with `serde`
    shadow_ty: DeriveInput,

    /// List of fields on the shadow type
    shadow_fields: Vec<syn::Ident>,
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
            src_fields: vec![],
            err: None,
        };

        // Fold thee shadow typee
        let mut output = fold_derive_input(&mut fold_shadow_ty, input);

        if let Some(err) = fold_shadow_ty.err {
            return Err(err);
        }

        output.ident = shadow_ty;
        output.attrs.retain(|attr| !Attribute::is_web_attribute(attr));

        Ok(Extract {
            ty,
            vis,
            shadow_ty: output,
            shadow_fields: fold_shadow_ty.src_fields,
        })
    }

    pub fn gen(&self) -> Result<TokenStream, String> {
        let dummy_const = self.dummy_const();
        let ty = &self.ty;
        let vis = &self.vis;
        let shadow_ty = &self.shadow_ty.ident;
        let shadow_def = self.shadow_def();
        let fields_1 = &self.shadow_fields;
        let fields_2 = &self.shadow_fields;

        Ok(quote! {
            #[allow(unused_variables, non_upper_case_globals)]
            const #dummy_const: () = {
                extern crate tower_web as __tw;

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

                        #ty {
                            #(#fields_1: shadow.#fields_2,)*
                        }
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
}

struct FoldShadowTy {
    /// Fields in original structure to use when converting to the shadow
    /// structure.
    src_fields: Vec<syn::Ident>,

    /// Any error encountered
    err: Option<String>,
}

impl syn::fold::Fold for FoldShadowTy {
    fn fold_fields_named(&mut self, mut fields: syn::FieldsNamed) -> syn::FieldsNamed {
        use syn::punctuated::Punctuated;
        use std::mem;

        macro_rules! try {
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

            let attrs = try!(Attribute::from_ast(&field.attrs));

            if attrs.is_empty() {
                self.src_fields.push(field.ident.clone().unwrap());
                fields.named.push(field);
            } else {
                unimplemented!();
            }
        }

        fields
    }

    fn fold_fields_unnamed(&mut self, _: syn::FieldsUnnamed) -> syn::FieldsUnnamed {
        unimplemented!("file={}; line={}", file!(), line!());
    }
}
