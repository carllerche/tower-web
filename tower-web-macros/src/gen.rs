use Service;

use proc_macro2::TokenStream;
use quote::TokenStreamExt;
use syn;

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

            let content_type = match route.rules.content_type.as_ref() {
                Some(content_type) => quote! { Some(#content_type) },
                None => quote! { None },
            };

            // Get the destination symbol
            let destination = if service.routes.len() >= 2 {
                route.destination_sym(quote!{ () })
            } else {
                quote! { () }
            };

            routes_fn.append_all(quote! {
                .route(#destination, #method, #path, {
                    #content_type
                        .and_then(|v| serializer.lookup(v))
                })
            });

            // For each action argument, generate the code necessary for
            // extracting the argument from the request.
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

            // Generate code for dispatching a request and handling the
            // response.
            if service.routes.len() > 1 {
                let wrap = route.destination_sym(quote! { response });
                dispatch_fn.append_all(quote! {
                    #destination => {
                        let response = ::tower_web::response::MapErr::new(
                            self.#ident(#(#args),*).into_future());

                        #wrap
                    }
                });
            } else {
                dispatch_fn.append_all(quote! {
                    #destination => {
                        ::tower_web::response::MapErr::new(
                            self.#ident(#(#args),*).into_future())
                    }
                });
            }
        }

        // If there are no routes, then some special work needs to be done to
        // make the generated code compile
        if service.routes.is_empty() {
            dispatch_fn.append_all(quote! {
                () => unreachable!(),
            });
        }

        let future_ty = match service.routes.len() {
            0 => {
                quote!(::tower_web::response::MapErr<<Result<String, ()> as ::tower_web::codegen::futures::IntoFuture>::Future>)
            }
            1 => {
                let ty = &service.routes[0].ret;
                quote!(::tower_web::response::MapErr<<#ty as ::tower_web::codegen::futures::IntoFuture>::Future>)
            }
            n => {
                let response_tys: Vec<_> = service
                    .routes
                    .iter()
                    .map(|route| {
                        let ty = &route.ret;
                        quote!{
                            ::tower_web::response::MapErr<<#ty as ::tower_web::codegen::futures::IntoFuture>::Future>
                        }
                    })
                    .collect();

                let either: syn::Type =
                    syn::parse_str(&format!("::tower_web::util::tuple::Either{}", n)).unwrap();

                quote! {
                    #either<#(#response_tys),*>
                }
            }
        };

        // Define `Resource` on the struct.
        tokens.append_all(quote! {
            impl ::tower_web::service::Resource for #ty {
                type Destination = #destination;
                type Buf = <Self::Response as ::tower_web::response::IntoResponse>::Buf;
                type Body = <Self::Response as ::tower_web::response::IntoResponse>::Body;
                type Response = <Self::Future as ::tower_web::codegen::futures::Future>::Item;
                type Future = #future_ty;

                fn routes<S>(&self, serializer: &S)
                    -> ::tower_web::routing::RouteSet<Self::Destination, S::ContentType>
                where S: ::tower_web::response::Serializer,
                {
                    use ::tower_web::routing;
                    #destination_use

                    routing::Builder::new()
                    #routes_fn
                    .build()
                }

                fn dispatch<In: ::tower_web::util::BufStream>(
                    &mut self,
                    destination: Self::Destination,
                    route_match: &::tower_web::routing::RouteMatch,
                    request: &::tower_web::codegen::http::Request<()>,
                    _payload: In
                ) -> Self::Future
                {
                    use ::tower_web::Extract;
                    use ::tower_web::codegen::CallSite;
                    use ::tower_web::codegen::futures::{/* future, stream, */ Future, Stream, IntoFuture};
                    use ::tower_web::response::IntoResponse;

                    #destination_use

                    match destination {
                        #dispatch_fn
                    }
                }
            }

            impl<U> ::tower_web::util::Chain<U> for #ty {
                type Output = (Self, U);

                fn chain(self, other: U) -> Self::Output {
                    (self, other)
                }
            }
        });
    }

    tokens.to_string()
}
