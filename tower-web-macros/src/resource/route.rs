use crate::resource::{Arg, Attributes, Signature, TyTree};

use proc_macro2::TokenStream;
use syn;
use quote::quote;

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

    pub fn template(&self) -> Option<&str> {
        self.attributes.template()
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
        // Because the arguments *might* be closed over into a trait object that may or may not be
        // send, the values must be extracted eagerly.
        //
        // To do this, the data is extracted from the futures into a tuple. The tuple is closed
        // over, which no longer has the problem of being `Send`.

        let args_outer = self.sig.args().iter().map(|arg| {
            let index = syn::Index::from(arg.index);
            quote! { __tw::extract::ExtractFuture::extract(args.#index) }
        });

        let args_inner = self.sig.args().iter().map(|arg| {
            let index = syn::Index::from(arg.index);
            quote! { args.#index }
        });

        let body = self.sig.dispatch(
            quote!(self.inner),
            args_inner);

        quote! {
            let args = args.into_inner();
            let args = (#(#args_outer,)*);
            #body
        }
    }

    pub fn handler_args_ty(&self) -> TokenStream {
        TyTree::new(self.args())
            .extract_args_ty()
    }

    /// The response future type
    pub fn future_ty(&self) -> TokenStream {
        self.sig.future_ty()
    }
}
