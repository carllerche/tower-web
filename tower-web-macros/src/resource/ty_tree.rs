//! Distribute a collection into a tree

use resource::Arg;

use syn;
use proc_macro2::{Span, TokenStream};

use std::cmp;

pub struct TyTree<'a, T: 'a> {
    data: &'a [T],
}

impl<'a, T> TyTree<'a, T> {
    pub fn new(data: &'a [T]) -> TyTree<'a, T> {
        TyTree { data }
    }

    fn map_reduce<F1, F2>(&self, map: F1, mut reduce: F2) -> TokenStream
    where F1: FnMut(&T) -> TokenStream,
          F2: FnMut(&[TokenStream]) -> TokenStream,
    {
        let mapped: Vec<_> = self.data.iter()
            .map(map)
            .collect();

        self::reduce(&mapped[..], &mut reduce)
    }
}

impl<'a> TyTree<'a, Arg> {
    pub fn extract_args_ty(&self) -> TokenStream {
        self.map_reduce(
            |arg| {
                let ty = &arg.ty;
                quote! { <#ty as __tw::extract::Extract>::Future }
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

                quote! {
                    <#ty as __tw::extract::Extract>::into_future(&{
                        let callsite = &callsites.#index;
                        route_match.extract_context(callsite)
                    })
                }
            },
            |tokens| {
                let join_ty = join_ty(tokens.len());
                quote! { #join_ty::new(#(#tokens),*) }
            })
    }
}

fn join_ty(len: usize) -> syn::Type {
    syn::parse_str(&format!("__tw::util::tuple::Join{}", len)).unwrap()
}

fn reduce<F>(mut src: &[TokenStream], f: &mut F) -> TokenStream
where F: FnMut(&[TokenStream]) -> TokenStream,
{
    let per_slot = cmp::max(1, src.len() / ::MAX_VARIANTS);

    if per_slot == 1 {
        f(src)
    } else {
        let mut reduced = vec![];

        while !src.is_empty() {
            reduced.push(reduce(&src[..per_slot], f));
            src = &src[per_slot..];
        }

        f(&reduced[..])
    }
}
