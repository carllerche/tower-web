use {Arg, Route, Service};
use attr::Attributes;

use syn;

/// Result of a parse
pub struct Parse {
    services: Vec<Service>,
}

/// Builds up the state while traversing the AST.
struct ImplWeb {
    services: Vec<Service>,
    curr_service: usize,
    curr_route: usize,
}

impl Parse {
    /// Parse an input source
    pub fn parse(input: &str) -> Parse {
        // Load the AST defining the web service
        let ast = syn::parse_str(input).unwrap();

        // AST transformer
        let mut v = ImplWeb::new();

        // Transfer the definition
        syn::fold::fold_file(&mut v, ast);

        Parse {
            services: v.services,
        }
    }

    /// Generate the service source
    pub fn generate(&self) -> String {
        ::gen::generate(&self.services)
    }
}

impl ImplWeb {
    /// Returns a new `ImplWeb` instance with default values.
    fn new() -> ImplWeb {
        ImplWeb {
            services: vec![],
            curr_service: 0,
            curr_route: 0,
        }
    }

    fn push_service(&mut self, self_ty: Box<syn::Type>) {
        self.curr_service = self.services.len();
        self.services.push(Service::new(self.curr_service, self_ty));
    }

    fn service(&mut self) -> &mut Service {
        &mut self.services[self.curr_service]
    }

    fn push_route(
        &mut self,
        ident: syn::Ident,
        ret: syn::Type,
        attrs: Attributes,
        args: Vec<Arg>,
    ) {
        let index = self.service().routes.len();
        self.curr_route = index;
        self.service()
            .routes
            .push(Route::new(index, ident, ret, attrs, args));
    }
}

impl syn::fold::Fold for ImplWeb {
    fn fold_item_impl(&mut self, item: syn::ItemImpl) -> syn::ItemImpl {
        assert!(
            item.trait_.is_none(),
            "trait impls must not be in impl_web! block"
        );

        self.push_service(item.self_ty.clone());

        syn::fold::fold_item_impl(self, item)
    }

    fn fold_impl_item_method(&mut self, mut item: syn::ImplItemMethod) -> syn::ImplItemMethod {
        use syn::ReturnType;

        let mut rules = Attributes::new();

        item.attrs.retain(|attr| !rules.process_attr(attr));

        if rules.is_empty() {
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
                            let param = rules.path_params.iter().position(|param| param == &ident);

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

        self.push_route(ident, ret, rules, args);

        item
    }
}
