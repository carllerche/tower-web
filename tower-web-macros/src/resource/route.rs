use resource::{Arg, Attributes, Signature, TyTree};

use proc_macro2::{TokenStream, Span};

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

    pub fn args(&self) -> &[Arg] {
        self.sig.args()
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

        let args = self.sig.args().iter().map(|arg| {
            let index = LitInt::new(arg.index as u64, IntSuffix::None, Span::call_site());
            quote! { __tw::extract::ExtractFuture::extract(args.#index) }
        });

        let body = self.sig.dispatch(
            quote!(self.inner.handler),
            args);

        quote! {
            let args = args.into_inner();
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
