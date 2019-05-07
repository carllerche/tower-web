use crate::resource::Arg;

use syn;
use proc_macro2::TokenStream;

use std::fmt;

pub(crate) struct Signature {
    /// Function identifier
    ident: syn::Ident,

    /// Function return type
    ret: syn::Type,

    /// True if the return value must be boxed
    box_ret: bool,

    /// Function arguments
    args: Vec<Arg>,

    /// True when an async function
    is_async: bool,
}

impl Signature {
    pub fn new(ident: syn::Ident, ret: syn::Type, args: Vec<Arg>, is_async: bool) -> Signature {
        let (ret, box_ret) = if is_async {
            // TODO: Determine if `Send` or not
            let tokens = quote! { Box<Future<Item = #ret, Error = __tw::Error> + Send> };
            let ret = syn::parse2(tokens).unwrap();

            (ret, true)
        } else {
            match ret {
                syn::Type::ImplTrait(obj) => {
                    (box_impl_trait(obj), true)
                }
                ret => (ret, false),
            }
        };

        Signature {
            ident,
            ret,
            box_ret,
            args,
            is_async,
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        &self.ident
    }

    pub fn args(&self) -> &[Arg] {
        &self.args[..]
    }

    pub fn ret(&self) -> &syn::Type {
        &self.ret
    }

    pub fn is_async(&self) -> bool {
        self.is_async
    }

    /// The response future type
    pub fn future_ty(&self) -> TokenStream {
        let ty = self.ret();

        if self.box_ret {
            quote! { #ty }
        } else {
            quote! {
                __tw::error::Map<<#ty as __tw::codegen::futures::IntoFuture>::Future>
            }
        }
    }

    pub fn dispatch<I>(&self, inner: TokenStream, args: I) -> TokenStream
    where I: Iterator<Item = TokenStream>,
    {
        let ident = self.ident();

        if self.is_async {
            let ty = self.ret();

            quote! {
                let inner = #inner.clone();
                let ret: #ty = __tw::codegen::async_await::async_to_box_future_send(async_move_hax! {
                    r#await!(inner.handler.#ident(#(#args),*))
                });
                ret
            }
        } else {
            let box_ret = if self.box_ret {
                let ty = self.ret();

                quote! { let ret: #ty = Box::new(ret); }
            } else {
                quote!()
            };

            quote! {
                let ret = __tw::error::Map::new(
                    __tw::codegen::futures::IntoFuture::into_future(#inner.handler.#ident(#(#args),*)));

                // If the return type must be boxed, the boxing happens here.
                #box_ret

                ret
            }
        }
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use quote::ToTokens;

        // TODO: Avoid escaping
        let ret = self.ret.clone().into_token_stream().to_string();

        fmt.debug_struct("Signature")
            .field("ident", &self.ident.to_string())
            .field("ret", &ret)
            .field("args", &self.args)
            .finish()
    }
}

fn box_impl_trait(obj: syn::TypeImplTrait) -> syn::Type {
    use syn::TypeParamBound::Trait;
    use syn::PathArguments::AngleBracketed;
    use syn::GenericArgument::Binding;

    // Try to identify the `Future` component
    let mut item = None;

    for bound in &obj.bounds {
        let bound = match bound {
            Trait(ref bound) => bound,
            _ => continue,
        };

        let segments = &bound.path.segments;
        let len = segments.len();

        assert!(len > 0);

        // `Future` would be the last segment
        if segments[len - 1].ident == "Future" {
            // Extract the bound's arguments
            let args = match segments[len - 1].arguments {
                AngleBracketed(ref args) => args,
                _ => continue,
            };

            // Find the `Item` binding argument
            for arg in &args.args {
                let arg = match arg {
                    Binding(ref arg) => arg,
                    _ => continue,
                };

                if arg.ident == "Item" {
                    item = Some(arg.ty.clone());
                    break;
                }
            }
        }
    }

    // TODO: Better error message
    let item = item.expect("failed to identify `impl T` as `Future`");

    syn::parse2(quote! {
        Box<Future<Item = #item, Error = __tw::Error> + Send>
    }).unwrap()
}
