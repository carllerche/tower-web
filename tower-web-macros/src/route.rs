use arg::Arg;
use attr::Attributes;
use ty_tree::TyTree;

use proc_macro2::{TokenStream, Span};
use syn;
use quote::ToTokens;

use std::fmt;

/// Represents a service route
pub(crate) struct Route {
    pub index: usize,

    /// Function identifier
    pub ident: syn::Ident,

    /// Function return type
    pub ret: syn::Type,

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
        Route {
            index,
            ident,
            ret,
            rules,
            args,
        }
    }

    /// Route builder fn call to add the route definition.
    pub fn build_route(&self, destination: TokenStream) -> TokenStream {
        let method = self.rules.method_expr();
        let path = self.rules.path_expr();

        quote! {
            .route(#destination, #method, #path)
        }
    }

    fn extract_args(&self) -> TokenStream {
        let args = self.args.iter().enumerate().map(|(i, arg)| {
            let extract = arg.extract();

            quote! {{
                let call_site = &route.#i;
                #extract
            }}
        });

        quote! { (#(#args),*) }
    }

    pub fn dispatch_fn(&self) -> TokenStream {
        TyTree::new(&self.args)
            .extract_args()
    }

    pub fn dispatch(&self) -> TokenStream {
        use syn::{LitInt, IntSuffix};

        let ident = &self.ident;
        let args = self.args.iter().enumerate().map(|(i, arg)| {
            let index = LitInt::new(arg.index as u64, IntSuffix::None, Span::call_site());
            quote! { args.#index.extract() }
        });

        quote! {
            let args = args.into_inner();
            MapErr::new(self.inner.handler.#ident(#(#args),*).into_future())
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
