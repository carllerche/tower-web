use resource::{Arg, Attributes, Catch, Signature, Route, Resource};

use proc_macro2::TokenStream;
use syn;

/// Result of a parse
pub struct Parse {
    resources: Vec<Resource>,
}

/// Builds up the state while traversing the AST.
struct ImplWeb {
    resources: Vec<Resource>,
}

impl Parse {
    /// Parse an input source
    pub fn parse(input: TokenStream) -> Parse {
        let ast = syn::parse2(input).unwrap();

        // AST transformer
        let mut v = ImplWeb::new();

        // Transfer the definition
        syn::fold::fold_file(&mut v, ast);

        for resource in &mut v.resources {
            resource.finalize();
        }

        Parse {
            resources: v.resources,
        }
    }

    /// Generate the resource source
    pub fn generate(&self) -> TokenStream {
        let impl_resources = self.resources.iter()
            .map(|resource| resource.gen());

        quote! {
            #(#impl_resources)*
        }
    }
}

impl ImplWeb {
    /// Returns a new `ImplWeb` instance with default values.
    fn new() -> ImplWeb {
        ImplWeb {
            resources: vec![],
        }
    }

    fn resource(&mut self) -> &mut Resource {
        self.resources.last_mut()
            .expect("no resources defined")
    }
}

impl syn::fold::Fold for ImplWeb {
    fn fold_item_impl(&mut self, item: syn::ItemImpl) -> syn::ItemImpl {
        assert!(
            item.trait_.is_none(),
            "trait impls must not be in impl_web! block"
        );

        let index = self.resources.len();
        self.resources.push(Resource::new(index, item.self_ty.clone()));

        syn::fold::fold_item_impl(self, item)
    }

    fn fold_impl_item_method(&mut self, mut item: syn::ImplItemMethod) -> syn::ImplItemMethod {
        use syn::ReturnType;

        let mut attributes = Attributes::new();

        item.attrs.retain(|attr| !attributes.process(attr));

        if attributes.is_empty() {
            // Not a web route, do no further processing.
            return item;
        }

        // Get the method name
        let ident = item.sig.ident.clone();

        // Get the return type
        let ret = match item.sig.decl.output {
            ReturnType::Type(_, ref ty) => (**ty).clone(),
            ReturnType::Default => syn::parse_str("()").unwrap(),
        };

        let mut args = vec![];

        // Process the args
        for arg in item.sig.decl.inputs.iter().skip(1) {
            use syn::{FnArg, Pat};

            match arg {
                FnArg::Captured(arg) => {
                    let index = args.len();
                    match arg.pat {
                        Pat::Ident(ref ident) => {
                            // Convert the identifier to a string
                            let ident = ident.ident.to_string();

                            // Check if the identifier matches any parameters
                            let param = attributes.path_params.iter()
                                .position(|param| param == &ident);

                            args.push(Arg::new(index, ident, param, arg.ty.clone()));
                        }
                        _ => {
                            // In this case, we should proceed without
                            // generating a call site as we cannot infer enough
                            // information about the argument.
                            args.push(Arg::ty_only(index, arg.ty.clone()));
                        }
                    }
                }
                _ => panic!("unexpected fn argument type = {:?}", arg),
            }
        }

        if attributes.is_route() {
            let index = self.resource().routes.len();
            let sig = Signature::new(ident, ret, args);
            let route = Route::new(index, sig, attributes);

            self.resource().routes.push(route);
        } else {
            let index = self.resource().catches.len();
            let sig = Signature::new(ident, ret, args);
            let catch = Catch::new(index, sig, attributes);

            self.resource().catches.push(catch);
        }

        item
    }
}
