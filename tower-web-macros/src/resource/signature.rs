use resource::Arg;

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

    args: Vec<Arg>,
}

impl Signature {
    pub fn new(ident: syn::Ident, ret: syn::Type, args: Vec<Arg>) -> Signature {
        let (ret, box_ret) = match ret {
            syn::Type::ImplTrait(obj) => {
                (box_impl_trait(obj), true)
            }
            ret => (ret, false),
        };

        Signature {
            ident,
            ret,
            box_ret,
            args,
        }
    }

    pub fn is_box_ret(&self) -> bool {
        self.box_ret
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

    /// The response future type
    pub fn future_ty(&self) -> TokenStream {
        let ty = self.ret();

        if self.is_box_ret() {
            quote! { #ty }
        } else {
            quote! {
                __tw::error::Map<<#ty as __tw::codegen::futures::IntoFuture>::Future>
            }
        }
    }

    pub fn dispatch<I>(&self, handler: TokenStream, args: I) -> TokenStream
    where I: Iterator<Item = TokenStream>,
    {
        let ident = self.ident();

        let box_ret = if self.is_box_ret() {
            let ty = self.ret();

            quote! { let ret: #ty = Box::new(ret); }
        } else {
            quote!()
        };

        quote! {
            let ret = __tw::error::Map::new(
                __tw::codegen::futures::IntoFuture::into_future(#handler.#ident(#(#args),*)));

            // If the return type must be boxed, the boxing happens here.
            #box_ret

            ret
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
    let mut has_send = false;

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
        } else if segments[len - 1].ident == "Send" {
            has_send = true;
        }
    }

    // TODO: Better error message
    let item = item.expect("failed to identify `impl T` as `Future`");

    let tokens = if has_send {
        quote! { Box<Future<Item = #item, Error = __tw::Error> + Send> }
    } else {
        quote! { Box<Future<Item = #item, Error = __tw::Error>> }
    };

    syn::parse2(tokens).unwrap()
}
