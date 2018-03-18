use {Service};

use quote::{ToTokens, Tokens};
use syn;

/// Generate the service implementations
pub fn generate(ast: &syn::File, services: &[Service]) -> String {
    // Tokens representing the output
    let mut tokens = ast.into_tokens();

    for service in services {
        let ty = &service.self_ty;

        let mut match_routes = Tokens::new();

        // Iterate over routes and generate the route matching code. For now,
        // this is incredibly naive.
        for route in &service.routes {
            let ident = &route.ident;
            let method = route.method.as_ref().unwrap().to_tokens();
            let path = route.path_lit.as_ref().unwrap();

            match_routes.append_all(quote! {
                if ::tower_web::route::matches(&request, &#method, #path) {
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

                fn call(&mut self, request: ::tower_web::codegen::http::Request<()>) -> Self::Future {
                    use ::tower_web::IntoResponse;
                    use ::tower_web::codegen::bytes::Bytes;
                    use ::tower_web::codegen::futures::{future, stream, Future, Stream};

                    #match_routes

                    let body = stream::once(Ok(Bytes::from_static(b"not found")));

                    let response = ::tower_web::codegen::http::Response::builder()
                        .status(404)
                        .header("content-type", "text/plain")
                        .body(Box::new(body) as Self::Body)
                        .unwrap();

                    Box::new(future::ok(response))
                }
            }
        });
    }

    tokens.to_string()
}
