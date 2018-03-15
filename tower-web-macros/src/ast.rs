use {Route, Service};

use quote::{ToTokens, Tokens};
use syn;

pub fn rewrite(input: &str) -> String {
    // Load the AST defining the web service
    let ast = syn::parse_str(input).unwrap();

    // AST transformer
    let mut v = ImplWeb::new();

    // Transfer the definition
    let ast = syn::fold::fold_file(&mut v, ast);

    let mut tokens = ast.into_tokens();

    for service in &v.services {
        let ty = &service.self_ty;

        let mut match_routes = Tokens::new();

        // Iterate over routes and generate the route matching code. For now,
        // this is incredibly naive.
        for route in &service.routes {
            let ident = &route.ident;

            match_routes.append_all(quote! {
                if true {
                    let response = self.#ident()
                        .into_response()
                        .map(|response| {
                            response.map(|body| {
                                // TODO: Log error
                                let body = body.map_err(|_| ::tower_web::Error::Internal);
                                Box::new(body) as Self::Body
                            })
                        });

                    return Box::new(response);
                }
            });
        }

        // Define `Resource` on the struct.
        tokens.append_all(quote! {
            impl ::tower_web::Resource for #ty {
                type Body = ::tower_web::codegen::BoxBody;
                type Future = ::tower_web::codegen::BoxResponse<Self::Body>;

                fn call(&mut self) -> Self::Future {
                    use ::tower_web::IntoResponse;
                    use ::tower_web::codegen::futures::{future, Future, Stream};

                    #match_routes

                    Box::new(future::err(::tower_web::Error::NotFound))
                }
            }
        });
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
