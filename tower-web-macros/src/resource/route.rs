use resource::{Arg, Attributes, Signature, TyTree};

use proc_macro2::{TokenStream, Span};
use syn;

/// Represents a resource route
#[derive(Debug)]
pub(crate) struct Route {
    pub index: usize,

    sig: Signature,

    pub attributes: Attributes,
}

impl Route {
    pub fn new(index: usize, sig: Signature, attributes: Attributes) -> Self {
        Route {
            index,
            sig,
            attributes,
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        self.sig.ident()
    }

    pub fn args(&self) -> &[Arg] {
        self.sig.args()
    }

    pub fn ret(&self) -> &syn::Type {
        self.sig.ret()
    }

    pub fn is_box_ret(&self) -> bool {
        self.sig.is_box_ret()
    }

    /// Route builder fn call to add the route definition.
    pub fn build_route(&self, destination: TokenStream) -> TokenStream {
        let method = self.attributes.method_expr();
        let path = self.attributes.path_expr();

        quote! {
            .insert({
                __tw::routing::Route::new(#destination)
                    .method(#method)
                    .path(#path)
            })
        }
    }

    pub fn dispatch_fn(&self) -> TokenStream {
        TyTree::new(self.args())
            .extract_args()
    }

    pub fn dispatch(&self) -> TokenStream {
        use syn::{LitInt, IntSuffix};

        let ident = self.ident();
        let args = self.sig.args().iter().map(|arg| {
            let index = LitInt::new(arg.index as u64, IntSuffix::None, Span::call_site());
            quote! { __tw::extract::ExtractFuture::extract(args.#index) }
        });

        let box_ret = if self.sig.is_box_ret() {
            let ty = self.ret();

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
        TyTree::new(self.args())
            .extract_args_ty()
    }

    /// The response future type
    pub fn future_ty(&self) -> TokenStream {
        let ty = self.ret();

        if self.is_box_ret() {
            quote! { #ty }
        } else {
            quote! {
                __tw::response::MapErr<<#ty as __tw::codegen::futures::IntoFuture>::Future>
            }
        }
    }

}
