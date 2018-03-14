use {Route, Service};

use quote::ToTokens;
use syn;

pub fn rewrite(input: &str) -> String {
    // Load the AST defining the web service
    let ast = syn::parse_str(input).unwrap();

    // AST transformer
    let mut v = ImplWeb::new();

    // Transfer the definition
    let ast = syn::fold::fold_file(&mut v, ast);

    /*
    println!("~~~~~ SERVICES ~~~~~~");
    println!("{:#?}", v.services);
    */

    let mut tokens = ast.into_tokens();

    for service in &v.services {
        let ty = &service.self_ty;

        // Get a single route
        // TODO: Route over all routes
        let route = &service.routes[0];
        let ident = &route.ident;
        let ret = &route.ret;

        tokens.append_all(quote! {
            impl ::tower_web::Resource for #ty {
                /*
                type Request = ::tower_web::codegen::http::Request<String>;
                type Response = ::tower_web::codegen::http::Response<String>;
                type Error = ();
                */
                type Future = ::tower_web::Map<#ret>;

                fn call(&mut self) -> Self::Future {
                    // TODO: Actually use the request object
                    let resp = self.#ident();
                    ::tower_web::Map::new(resp)
                }
            }
        });

        /*
        // Get a single route
        let route = &service.routes[0];
        let ident = &route.ident;
        let ret = &route.ret;

        tokens.append_all(quote! {
            impl ::tower_web::codegen::tower::Service for #ty {
                type Request = ::tower_web::codegen::http::Request<String>;
                type Response = ::tower_web::codegen::http::Response<String>;
                type Error = ();
                type Future = ::tower_web::Map<#ret>;

                fn poll_ready(&mut self) -> ::tower_web::codegen::futures::Poll<(), Self::Error> {
                    // TODO: Implement
                    Ok(().into())
                }

                fn call(&mut self, _request: Self::Request) -> Self::Future {
                    // TODO: Actually use the request object
                    let resp = self.#ident();
                    ::tower_web::Map::new(resp)
                }
            }
        });
        */
    }

    tokens.to_string()
}

struct ImplWeb {
    services: Vec<Service>,
    curr_service: usize,
    curr_route: usize,
}

impl ImplWeb {
    fn new() -> ImplWeb {
        ImplWeb {
            services: vec![],
            curr_service: 0,
            curr_route: 0,
        }
    }

    fn push_service(&mut self, self_ty: Box<syn::Type>) {
        self.curr_service = self.services.len();
        self.services.push(Service::new(self_ty));
    }

    fn service(&mut self) -> &mut Service {
        &mut self.services[self.curr_service]
    }

    fn push_route(&mut self, ident: syn::Ident, ret: syn::Type) {
        self.curr_route = self.service().routes.len();
        self.service().routes.push(Route::new(ident, ret));
    }

    fn route(&mut self) -> &mut Route {
        let curr = self.curr_route;
        &mut self.service().routes[curr]
    }
}

impl syn::fold::Fold for ImplWeb {
    fn fold_item_impl(&mut self, item: syn::ItemImpl) -> syn::ItemImpl {
        assert!(item.trait_.is_none(), "trait impls must not be in impl_web! block");

        self.push_service(item.self_ty.clone());

        syn::fold::fold_item_impl(self, item)
    }

    fn fold_impl_item_method(&mut self, mut item: syn::ImplItemMethod) -> syn::ImplItemMethod {
        use syn::ReturnType;

        // Get the method name
        let ident = item.sig.ident;

        // println!("ARGS = {:#?}", item.sig.decl.inputs);

        // Get the return type
        let ret = match item.sig.decl.output {
            ReturnType::Type(_, ref ty) => (**ty).clone(),
            ReturnType::Default => unimplemented!(),
        };

        self.push_route(ident, ret);

        item.attrs.retain(|attr| {
            !self.route().process_attr(attr)
        });

        item
    }
}
