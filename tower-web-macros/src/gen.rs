use Service;

use proc_macro2::TokenStream;
use quote::TokenStreamExt;

/// Generate the service implementations
pub fn generate(services: &[Service]) -> String {
    // Tokens representing the output
    let mut tokens = TokenStream::new();

    for service in services {
        let ty = &service.self_ty;
        let destination = service.destination_ty();
        let destination_use = service.destination_ty_use();

        let mut routes_fn = TokenStream::new();
        let mut dispatch_fn = TokenStream::new();

        for route in &service.routes {
            let ident = &route.ident;
            let method = route.rules.method.as_ref().unwrap().to_tokens();
            let path = route.rules.path_lit.as_ref().unwrap();

            // Get the destination symbol
            let destination = if service.routes.len() >= 2 {
                route.destination_sym()
            } else {
                quote! { () }
            };

            routes_fn.append_all(quote! {
                .route(#destination, #method, #path)
            });

            let args: Vec<_> = route
                .args
                .iter()
                .map(|arg| {
                    let ty = &arg.ty;

                    // TODO: Don't unwrap

                    match arg.ident {
                        Some(ref ident) => {
                            let param = match arg.param {
                                Some(idx) => quote!(Some(#idx)),
                                None => quote!(None),
                            };

                            quote!({
                                let callsite = CallSite::new(#ident, #param);
                                <#ty>::callsite_extract(
                                    &callsite,
                                    &route_match,
                                    &request).unwrap()
                            })
                        }
                        None => quote!(<#ty>::extract(&route_match, &request).unwrap()),
                    }
                })
                .collect();

            dispatch_fn.append_all(quote! {
                #destination => {
                    let response = self.#ident(#(#args),*)
                        .into_response()
                        .map(|response| {
                            response.map(|body| {
                                // TODO: Log error
                                let body = body.map_err(|_| ::tower_web::Error::Internal);
                                Box::new(body) as Self::Body
                            })
                        });

                    Box::new(response) as Self::Future
                }
            });
        }

        // If there are no routes, then some special work needs to be done to
        // make the generated code compile
        if service.routes.is_empty() {
            dispatch_fn.append_all(quote! {
                () => unreachable!(),
            });
        }

        // Define `Resource` on the struct.
        tokens.append_all(quote! {
            impl ::tower_web::Resource for #ty {
                type Destination = #destination;
                type Body = ::tower_web::codegen::BoxBody;
                type Future = ::tower_web::codegen::BoxResponse<Self::Body>;

                fn routes(&self) -> ::tower_web::routing::RouteSet<Self::Destination> {
                    use ::tower_web::routing;
                    #destination_use

                    routing::Builder::new()
                    #routes_fn
                    .build()
                }

                fn dispatch(&mut self,
                            destination: Self::Destination,
                            route_match: &::tower_web::routing::RouteMatch,
                            request: &::tower_web::codegen::http::Request<()>)
                    -> Self::Future
                {
                    use ::tower_web::{IntoResponse, Extract, CallSite};
                    // use ::tower_web::codegen::bytes::Bytes;
                    use ::tower_web::codegen::futures::{/* future, stream, */ Future, Stream};
                    #destination_use

                    match destination {
                        #dispatch_fn
                    }
                }
            }

            impl<U> ::tower_web::resource::Chain<U> for #ty {
                type Resource = (Self, U);

                fn chain(self, other: U) -> Self::Resource {
                    (self, other)
                }
            }
        });
    }

    tokens.to_string()
}
