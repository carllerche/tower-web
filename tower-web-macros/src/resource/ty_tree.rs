//! Distribute a collection into a tree

use resource::{Arg, Route};

use syn;
use proc_macro2::{Span, TokenStream};

use std::cmp;

pub(crate) struct TyTree<'a, T: 'a> {
    data: &'a [T],
}

impl<'a, T> TyTree<'a, T> {
    pub fn new(data: &'a [T]) -> TyTree<'a, T> {
        TyTree { data }
    }

    pub fn map_reduce<F1, F2, R>(&self, map: F1, mut reduce: F2) -> R
    where F1: FnMut(&T) -> R,
          F2: FnMut(&[R]) -> R,
          R: Clone,
    {
        let mapped: Vec<_> = self.data.iter()
            .map(map)
            .collect();

        self::reduce(&mapped[..], &mut reduce)
    }

    pub fn map_either<F>(&self, map: F) -> TokenStream
    where F: FnMut(&T) -> TokenStream,
    {
        self.map_reduce(
            map,
            |tokens| {
                let either_ty = either_ty(tokens.len());
                quote! { #either_ty<#(#tokens),*> }
            })
    }
}

impl<'a> TyTree<'a, Arg> {
    pub fn extract_args_ty(&self) -> TokenStream {
        self.map_reduce(
            |arg| {
                let ty = &arg.ty;
                quote! { <#ty as __tw::extract::Extract<B>>::Future }
            },
            |tokens| {
                let join_ty = join_ty(tokens.len());
                quote! { #join_ty<#(#tokens),*> }
            })
    }

    pub fn extract_args(&self) -> TokenStream {
        use syn::{LitInt, IntSuffix};

        self.map_reduce(
            |arg| {
                let ty = &arg.ty;
                let index = LitInt::new(arg.index as u64, IntSuffix::None, Span::call_site());

                quote! {{
                    let context = __tw::extract::Context::new(
                        &route_match,
                        &callsites.#index.0);

                    if callsites.#index.1 {
                        <#ty as __tw::extract::Extract<B>>::extract_body(
                            &context,
                            body.take().unwrap())
                    } else {
                        <#ty as __tw::extract::Extract<B>>::extract(&context)
                    }
                }}
            },
            |tokens| {
                let join_ty = join_ty(tokens.len());
                quote! { #join_ty::new(#(#tokens),*) }
            })
    }
}

impl<'a> TyTree<'a, Route> {
    pub fn handler_future_ty(&self) -> TokenStream {
        self.map_either(|route| route.future_ty())
    }

    pub fn handler_args_ty(&self) -> TokenStream {
        self.map_either(|route| route.handler_args_ty())
    }
}

fn join_ty(len: usize) -> syn::Type {
    syn::parse_str(&format!("__tw::util::tuple::Join{}", len)).unwrap()
}

fn either_ty(len: usize) -> syn::Type {
    syn::parse_str(&format!("__tw::util::tuple::Either{}", len)).unwrap()
}

// TODO: Remove clone?
fn reduce<F, R>(mut src: &[R], f: &mut F) -> R
where F: FnMut(&[R]) -> R,
      R: Clone,
{
    let per_slot = cmp::max(1, src.len() / ::MAX_VARIANTS);
    let mut rem = 0;

    if src.len() > ::MAX_VARIANTS {
        rem = src.len() % ::MAX_VARIANTS;
    }

    let mut reduced = vec![];

    while !src.is_empty() {
        let mut n = per_slot;

        if rem > 0 {
            n += 1;
            rem -= 1;
        }

        assert!(n > 0);

        if n == 1 {
            reduced.push(src[0].clone());
        } else {
            reduced.push(reduce(&src[..n], f));
        }

        src = &src[n..];
    }

    f(&reduced[..])
}
