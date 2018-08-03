use resource::{Arg, Attributes, TyTree};

use proc_macro2::{TokenStream, Span};
use syn;

use std::fmt;

/// Represents a resource route
pub(crate) struct Route {
    pub index: usize,

    /// Function identifier
    pub ident: syn::Ident,

    /// Function return type
    pub ret: syn::Type,

    /// True if the return value must be boxed
    pub box_ret: bool,

    pub rules: Attributes,

    pub args: Vec<Arg>,
}

impl Route {
    pub fn new(
        index: usize,
        ident: syn::Ident,
        ret: syn::Type,
        rules: Attributes,
        args: Vec<Arg>,
    ) -> Self {
        let (ret, box_ret) = match ret {
            syn::Type::ImplTrait(obj) => {
                (box_impl_trait(obj), true)
            }
            ret => (ret, false),
        };

        Route {
            index,
            ident,
            ret,
            box_ret,
            rules,
            args,
        }
    }

    /// Route builder fn call to add the route definition.
    pub fn build_route(&self, destination: TokenStream) -> TokenStream {
        let method = self.rules.method_expr();
        let path = self.rules.path_expr();

        quote! {
            .insert({
                __tw::routing::Route::new(#destination)
                    .method(#method)
                    .path(#path)
            })
        }
    }

    pub fn dispatch_fn(&self) -> TokenStream {
        TyTree::new(&self.args)
            .extract_args()
    }

    pub fn dispatch(&self) -> TokenStream {
        use syn::{LitInt, IntSuffix};

        let ident = &self.ident;
        let args = self.args.iter().map(|arg| {
            let index = LitInt::new(arg.index as u64, IntSuffix::None, Span::call_site());
            quote! { __tw::extract::ExtractFuture::extract(args.#index) }
        });

        let box_ret = if self.box_ret {
            let ty = &self.ret;

            quote! { let ret: #ty = Box::new(ret); }
        } else {
            quote!()
        };

        quote! {
            let args = args.into_inner();
            let ret = __tw::response::MapErr::new(
                __tw::codegen::futures::IntoFuture::into_future(self.inner.handler.#ident(#(#args),*)));

            // If the return type must be boxed, the boxing happens here.
            #box_ret

            ret
        }
    }

    pub fn handler_args_ty(&self) -> TokenStream {
        TyTree::new(&self.args)
            .extract_args_ty()
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
