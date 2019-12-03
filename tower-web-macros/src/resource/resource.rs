use crate::resource::{Route, Catch, TyTree};

use quote::{TokenStreamExt, ToTokens};
use proc_macro2::{Ident, Span, TokenStream};
use syn;
use quote::quote;

#[derive(Debug)]
pub(crate) struct Resource {
    /// The resource index in the `impl_web` block
    index: usize,

    /// The type implementing `Resource`
    pub self_ty: Box<syn::Type>,

    /// Resource generics
    generics: syn::Generics,

    /// The route handlers implemented by `Resource`
    pub routes: Vec<Route>,

    /// The catch handlers implemented by `Resource`.
    pub catches: Vec<Catch>,

    /// Route destinations
    destinations: Vec<Destination>,
}

#[derive(Debug, Clone)]
struct Destination {
    path: Vec<TokenStream>,
}

impl Resource {
    pub fn new(index: usize, item: &syn::ItemImpl) -> Resource {
        let self_ty = item.self_ty.clone();
        let generics = item.generics.clone();

        Resource {
            index,
            self_ty,
            generics,
            routes: vec![],
            catches: vec![],
            destinations: vec![],
        }
    }

    /// Generate the implementation
    pub fn gen(&self) -> TokenStream {
        assert_eq!(self.routes.len(), self.destinations.len());
        assert!(self.catches.len() <= 1, "at most one catch handler per resource");

        let resource_impl = if self.routes.is_empty() {
            self.gen_empty_impl()
        } else {
            self.gen_impl()
        };

        let dummy_const = self.dummy_const();
        let ty = &self.self_ty;
        let generics = self.resource_generics();
        let where_predicates = self.where_predicates();

        quote! {
            #[allow(warnings)]
            const #dummy_const: () = {
                use tower_web as __tw;

                #resource_impl

                impl<__U, #generics> __tw::util::Chain<__U> for #ty
                where
                    #where_predicates
                {
                    type Output = (Self, __U);

                    fn chain(self, other: __U) -> Self::Output {
                        (self, other)
                    }
                }
            };
        }
    }

    fn gen_impl(&self) -> TokenStream {
        let ty = &self.self_ty;

        let generics = self.resource_generics();
        let generic_idents = self.resource_generic_idents();
        let where_predicates = self.where_predicates();

        // The destination type is `Either{N}` where `N` is the number of routes
        // or ::MAX_VARIANTS, which ever is smaller. In the latter case, some
        // routes will require nested `Either{N}` variants to uniquely identify
        // them.
        let destination_ty = self.destination_ty();

        // The Resource must store a `CallSite` instance for each arg for each
        // route.
        let callsites_def = self.callsites_def();

        // The Resource must store the response content type for each route
        let content_types_def = self.content_types_def();

        // Adds lines for each route, adding the route the route builder.
        let build_routes_fn = self.build_routes_fn();

        let dispatch_fn = self.dispatch_fn();

        let handler_future_ty = self.handler_future_ty();
        let extract_future_ty = self.extract_future_ty();

        let catch_impl = self.catch_impl();
        let catch_future_ty = self.catch_future_ty();
        let catch_fn = self.catch_fn();
        let catch_into_response = self.catch_into_response();

        let match_extract = self.match_extract();
        let match_into_response = self.match_into_response();

        // Define `Resource` on the struct.
        quote! {
            macro_rules! try_ready {
                ($e:expr) => {{
                    match $e {
                        Ok(__tw::codegen::futures::Async::Ready(t)) => t,
                        Ok(__tw::codegen::futures::Async::NotReady) => {
                            return Ok(__tw::codegen::futures::Async::NotReady)
                        }
                        Err(e) => return Err(From::from(e)),
                    }
                }}
            }

            pub struct __GeneratedResource<S, B, T>
            where S: __tw::response::Serializer,
                  B: __tw::util::BufStream,
            {
                inner: ::std::sync::Arc<__Inner<S, T>>,
                _p: ::std::marker::PhantomData<B>,
            }

            struct __Inner<S, T>
            where S: __tw::response::Serializer,
            {
                handler: T,
                callsites: CallSites,
                content_types: ContentTypes<S>,
                serializer: S,
            }

            #callsites_def
            #content_types_def

            impl<S, B, T> __GeneratedResource<S, B, T>
            where S: __tw::response::Serializer,
                  B: __tw::util::BufStream,
            {
                fn new(handler: T, serializer: S) -> Self {
                    let callsites = CallSites::new::<B>();
                    let content_types = ContentTypes::new(&serializer);

                    callsites.verify();

                    let inner = ::std::sync::Arc::new(__Inner {
                        handler,
                        callsites,
                        content_types,
                        serializer,
                    });

                    __GeneratedResource {
                        inner,
                        _p: ::std::marker::PhantomData,
                    }
                }
            }

            impl<S, B, T> Clone for __GeneratedResource<S, B, T>
            where S: __tw::response::Serializer,
                  B: __tw::util::BufStream,
            {
                fn clone(&self) -> Self {
                    let inner = self.inner.clone();
                    __GeneratedResource {
                        inner,
                        _p: ::std::marker::PhantomData,
                    }
                }
            }

            impl<__S, __B, #generics> __tw::routing::IntoResource<__S, __B> for #ty
            where __S: __tw::response::Serializer,
                  __B: __tw::util::BufStream,
                  #where_predicates
            {
                type Destination = #destination_ty;
                type Resource = __GeneratedResource<__S, __B, #ty>;

                fn routes(&self) -> __tw::routing::RouteSet<Self::Destination> {
                    __tw::routing::Builder::new()
                    #build_routes_fn
                    .build()
                }

                fn into_resource(self, serializer: __S) -> Self::Resource {
                    __GeneratedResource::new(self, serializer)
                }
            }

            impl<__S, __B, #generics> __tw::routing::Resource for __GeneratedResource<__S, __B, #ty>
            where __S: __tw::response::Serializer,
                  __B: __tw::util::BufStream,
                  #where_predicates
            {
                // The destination token is used to identify which action to
                // call
                type Destination = #destination_ty;

                type RequestBody = __B;

                // The response body's chunk type.
                type Buf = <Self::Body as __tw::util::BufStream>::Item;

                // The reesponse body type
                type Body = <Self::Future as __tw::routing::ResourceFuture>::Body;

                // Future representing processing the request.
                type Future = __ResponseFuture<__S, __B, #ty, #generic_idents>;

                fn dispatch(
                    &mut self,
                    destination: Self::Destination,
                    route_match: &__tw::routing::RouteMatch,
                    body: Self::RequestBody,
                ) -> Self::Future
                {
                    let mut body = Some(body);
                    #dispatch_fn
                }
            }

            #catch_impl

            pub struct __ResponseFuture<__S, __B, __T, #generics>
            where __S: __tw::response::Serializer,
                  __B: __tw::util::BufStream,
                  #where_predicates
            {
                state: State<__B, #generic_idents>,
                inner: ::std::sync::Arc<__Inner<__S, __T>>,
            }

            // Tracks the resource's response state. At a high level, the steps
            // to process a dispatch are:
            //
            // 1) Extract arguments
            // 2) Call handler
            // 3) Wait on response future
            // 4) Serialize.
            //
            // Of these steps, 1) and 3) are asynchronous.
            enum State<__B, #generics>
            where
                __B: __tw::util::BufStream,
                #where_predicates
            {
                Extract(#extract_future_ty),
                Response(#handler_future_ty),
                Error(#catch_future_ty),
                // PhantomData is needed for when the resource has no routes
                // that use the body component.
                Invalid(::std::marker::PhantomData<(__B, #generic_idents)>),
            }

            impl<__S, __B, #generics> __tw::routing::ResourceFuture for __ResponseFuture<__S, __B, #ty, #generic_idents>
            where __S: __tw::response::Serializer,
                  __B: __tw::util::BufStream,
                  #where_predicates
            {
                type Body = ResponseBody<#generic_idents>;

                fn poll_response(&mut self, request: &__tw::codegen::http::Request<()>)
                    -> __tw::codegen::futures::Poll<__tw::codegen::http::Response<Self::Body>, __tw::Error>
                {
                    loop {
                        // TODO: Clean this up!
                        let mut err = None;

                        match self.state {
                            State::Extract(ref mut extract_future) => {
                                try_ready!(__tw::codegen::futures::Future::poll(extract_future));
                            }
                            State::Response(ref mut response) => {
                                let response = match __tw::codegen::futures::Future::poll(response) {
                                    Ok(__tw::codegen::futures::Async::Ready(response)) => {
                                        let response = match response {
                                            #match_into_response
                                        };

                                        match response {
                                            Ok(response) => return Ok(__tw::codegen::futures::Async::Ready(response)),
                                            Err(e) => {
                                                err = Some(e);
                                            }
                                        }
                                    }
                                    Ok(__tw::codegen::futures::Async::NotReady) => {
                                        return Ok(__tw::codegen::futures::Async::NotReady);
                                    }
                                    Err(e) => {
                                        err = Some(e);
                                    }
                                };
                            }
                            State::Error(ref mut response) => {
                                return Ok(__tw::codegen::futures::Async::Ready({
                                    let res = __tw::codegen::futures::Future::poll(response);
                                    let response = try_ready!(res);
                                    let response = #catch_into_response;
                                    response?
                                }));
                            }
                            State::Invalid(_) => unreachable!(),
                        }

                        if let Some(err) = err.take() {
                            self.state = State::Error({
                                #catch_fn
                            });
                        } else {
                            let args = match ::std::mem::replace(&mut self.state, State::Invalid(::std::marker::PhantomData)) {
                                State::Extract(fut) => fut,
                                _ => unreachable!(),
                            };

                            self.state = State::Response(match args {
                                #match_extract
                            });
                        }
                    }
                }
            }

            /// Response body
            pub struct ResponseBody<#generics>(
                ::std::result::Result<
                    <<#handler_future_ty as __tw::codegen::futures::Future>::Item as __tw::response::Response>::Body,
                    <<#catch_future_ty as __tw::codegen::futures::Future>::Item as __tw::response::Response>::Body,
                >, ::std::marker::PhantomData<(#generic_idents)>);

            /// Response buf
            pub struct ResponseBuf<#generics>(
                ::std::result::Result<
                    <<#handler_future_ty as __tw::codegen::futures::Future>::Item as __tw::response::Response>::Buf,
                    <<#catch_future_ty as __tw::codegen::futures::Future>::Item as __tw::response::Response>::Buf,
                >, ::std::marker::PhantomData<(#generic_idents)>);

            impl<#generics> __tw::util::BufStream for ResponseBody<#generic_idents> {
                type Item = ResponseBuf<#generic_idents>;
                type Error = __tw::Error;

                fn poll(&mut self) -> __tw::codegen::futures::Poll<Option<Self::Item>, Self::Error> {
                    match self.0 {
                        Ok(ref mut b) => {
                            let buf = try_ready!(b.poll());
                            Ok(buf.map(|buf| {
                                ResponseBuf(Ok(buf), ::std::marker::PhantomData)
                            }).into())
                        }
                        Err(ref mut b) => {
                            let buf = try_ready!(b.poll());
                            Ok(buf.map(|buf| {
                                ResponseBuf(Err(buf), ::std::marker::PhantomData)
                            }).into())
                        }
                    }
                }

                fn size_hint(&self) -> __tw::util::buf_stream::SizeHint {
                    match self.0 {
                        Ok(ref b) => b.size_hint(),
                        Err(ref b) => b.size_hint(),
                    }
                }
            }

            impl<#generics> ::std::fmt::Debug for ResponseBody<#generic_idents> {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    fmt.debug_struct("ResponseBody")
                        .finish()
                }
            }

            // TODO: Implement default fns
            impl<#generics> __tw::codegen::bytes::Buf for ResponseBuf<#generic_idents> {
                fn remaining(&self) -> usize {
                    match self.0 {
                        Ok(ref b) => b.remaining(),
                        Err(ref b) => b.remaining(),
                    }
                }

                fn bytes(&self) -> &[u8] {
                    match self.0 {
                        Ok(ref b) => b.bytes(),
                        Err(ref b) => b.bytes(),
                    }
                }

                fn advance(&mut self, cnt: usize) {
                    match self.0 {
                        Ok(ref mut b) => b.advance(cnt),
                        Err(ref mut b) => b.advance(cnt),
                    }
                }
            }

            impl<#generics> ::std::fmt::Debug for ResponseBuf<#generic_idents> {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    fmt.debug_struct("ResponseBuf")
                        .finish()
                }
            }
        }
    }

    /// Generate code for a resource with no routes
    fn gen_empty_impl(&self) -> TokenStream {
        let ty = &self.self_ty;
        let generics = self.resource_generics();
        let where_predicates = self.where_predicates();

        quote! {
            impl<__S, __B, #generics> __tw::routing::IntoResource<__S, __B> for #ty
            where __S: __tw::response::Serializer,
                  __B: __tw::util::BufStream,
                  #where_predicates
            {
                type Destination = ();
                type Resource = __tw::routing::Unit<__B>;

                fn routes(&self) -> __tw::routing::RouteSet<Self::Destination> {
                    __tw::routing::RouteSet::new()
                }

                fn into_resource(self, serializer: __S) -> Self::Resource {
                    __tw::routing::Unit::new()
                }
            }
        }
    }

    fn self_ident(&self) -> Option<&syn::Ident> {
        match *self.self_ty {
            syn::Type::Path(ref type_path) => {
                let segments = &type_path.path.segments;
                let len = segments.len();
                Some(&segments[len-1].ident)
            }
            _ => None,
        }
    }

    fn resource_generics(&self) -> TokenStream {
        let generics = self.generics.params.iter();

        quote! {
            #(#generics),*
        }
    }

    fn resource_generic_idents(&self) -> TokenStream {
        use syn::GenericParam::Type;

        let idents = self.generics.params.iter()
            .map(|param| {
                match *param {
                    Type(ref type_param) => {
                        &type_param.ident
                    }
                    _ => unimplemented!(),
                }
            });

        quote! {
            #(#idents),*
        }
    }

    fn where_predicates(&self) -> TokenStream {
        if let Some(ref clause) = self.generics.where_clause {
            let predicates = clause.predicates.iter();

            quote! {
                #(#predicates),*
            }
        } else {
            quote! {
            }
        }
    }

    fn dummy_const(&self) -> syn::Ident {
        // A (slightly) helpful string snippet to identify *which* service
        // implementation this scope is for
        let helpful = dummy_const_ident(&self.self_ty);

        // The actual const identifieer uses the service index to ensure the
        // const name is uniquee
        Ident::new(&format!("__IMPL_WEB_{}_FOR_{}", self.index, helpful), Span::call_site())
    }

    /// The resource destination type.
    fn destination_ty(&self) -> TokenStream {
        TyTree::new(&self.routes[..])
            .map_either(|_| quote!(()))
    }

    fn callsites_def(&self) -> TokenStream {
        let fields = self.routes.iter().enumerate()
            .map(|(i, route)| {
                let name = route_n(i);
                let tys = (0..route.args().len())
                    .map(|_| quote!((__tw::codegen::CallSite, bool),));

                quote! { #name: (#(#tys)*) }
            });

        let init = self.routes.iter().enumerate()
            .map(|(i, route)| {
                let name = route_n(i);
                let init = route.args().iter()
                    .map(|arg| {
                        let new = arg.new_callsite();
                        let ty = &arg.ty;

                        quote! {
                            {
                                let callsite = #new;
                                let requires_body =
                                    <#ty as __tw::extract::Extract<B>>::requires_body(&callsite);

                                (callsite, requires_body)
                            },
                        }
                    });

                quote! { #name: (#(#init)*) }
            });

        let verify = self.routes.iter()
            .map(|route| {
                let name = route_n(route.index);
                let verify = route.args().iter()
                    .map(|arg| {
                        let index = syn::Index::from(arg.index);

                        quote! {
                            match (used_body, self.#name.#index.1) {
                                (false, true) => {
                                    used_body = true;
                                }
                                (true, true) => {
                                    panic!("unimplemented: multi body extract");
                                }
                                _ => {}
                            }
                        }
                    });

                quote! {
                    let mut used_body = false;
                    #(#verify)*
                }
            });

        quote! {
            struct CallSites {
                #(#fields),*
            }

            impl CallSites {
                fn new<B: __tw::util::BufStream>() -> CallSites {
                    CallSites {
                        #(#init),*
                    }
                }

                fn verify(&self) {
                    #(#verify)*
                }
            }
        }
    }

    fn content_types_def(&self) -> TokenStream {
        let num = self.routes.len();
        let init = self.routes.iter()
            .map(|route| {
                if let Some(ref content_type) = route.attributes.content_type {
                    quote!({
                        match serializer.lookup(#content_type) {
                            Some(content_type) => ContentType::Serializable(content_type),
                            None => {
                                let value = __tw::codegen::http::header::HeaderValue::from_str(#content_type)
                                    .unwrap();
                                ContentType::Unknown(Some(value))
                            }
                        }
                    })
                } else {
                    quote!(ContentType::Unknown(None))
                }
            });

        quote! {
            struct ContentTypes<S: __tw::response::Serializer> {
                content_types: [ContentType<S::Format>; #num],
            }

            enum ContentType<T> {
                Serializable(__tw::response::ContentType<T>),
                Unknown(Option<__tw::codegen::http::header::HeaderValue>),
            }

            impl<S> ContentTypes<S>
            where S: __tw::response::Serializer,
            {
                fn new(serializer: &S) -> Self {
                    ContentTypes {
                        content_types: [#(#init),*],
                    }
                }
            }
        }
    }

    fn dispatch_fn(&self) -> TokenStream {
        assert!(!self.routes.is_empty());

        let branches = self.destination_syms(|route, destination| {
            let left = destination.build_default();
            let dispatch_fn = destination.build(route.dispatch_fn());
            let route_n = Ident::new(
                &format!("route_{}", route.index),
                Span::call_site());

            quote! {
                #left => {
                    let callsites = &self.inner.callsites.#route_n;
                    #dispatch_fn
                }
            }
        });

        quote! {
            let either = match destination {
                #branches
            };

            let state = State::Extract(either);
            let inner = self.inner.clone();

            __ResponseFuture {
                state,
                inner,
            }
        }
    }

    fn catch_impl(&self) -> TokenStream {
        quote!()
    }

    fn catch_future_ty(&self) -> TokenStream {
        if let Some(catch) = self.catches.get(0) {
            catch.future_ty()
        } else {
            quote! {
                __tw::codegen::futures::future::FutureResult<
                    ::std::string::String,
                    __tw::Error>
            }
        }
    }

    fn catch_fn(&self) -> TokenStream {
        if let Some(catch) = self.catches.get(0) {
            catch.dispatch()
        } else {
            quote!(__tw::codegen::futures::future::err(err))
        }
    }

    fn match_extract(&self) -> TokenStream {
        self.destination_syms(|route, destination| {
            let dispatch = route.dispatch();

            let left = destination.build(quote!(args));
            let right = destination.build(quote! {{ #dispatch }});

            quote! {
                #left => #right,
            }
        })
    }

    fn match_into_response(&self) -> TokenStream {
        let set_resource_name = match self.self_ident() {
            Some(ident) => {
                let name = ident.to_string();
                quote!(context.set_resource_name(#name);)
            }
            None => quote!(),
        };

        self.destination_syms(|route, destination| {
            let fn_ident = route.ident().to_string();
            let left = destination.build(quote!(response));
            let map = destination.build(quote!(body));
            let idx = route.index;

            let set_template = match route.template() {
                Some(template) => {
                    quote! {
                        context.set_template(#template);
                    }
                }
                None => quote!(),
            };

            quote! {
                #left => {
                    let mut context = __tw::response::Context::new(
                        request, &self.inner.serializer);

                    #set_template

                    match self.inner.content_types.content_types[#idx] {
                        ContentType::Serializable(ref v) => {
                            context.set_default_format(v.format());
                            context.set_content_type(v.header());
                        }
                        ContentType::Unknown(ref header) => {
                            if let Some(ref header) = *header {
                                context.set_content_type(header);
                            } else {
                                if let Some(extension) = request.uri().path().split(".").last() {
                                    if let Some(content_type) = __tw::util::mime_types::BY_EXTENSION.get(extension) {
                                        context.set_content_type(content_type);
                                    }
                                }
                            }
                        }
                    }

                    context.set_resource_mod(module_path!());
                    #set_resource_name
                    context.set_handler_name(#fn_ident);

                    __tw::response::Response::into_http(response, &context).map(|response| {
                        response.map(|body| ResponseBody(Ok(#map), ::std::marker::PhantomData))
                    })
                }
            }
        })
    }

    fn catch_into_response(&self) -> TokenStream {
        quote!({
            let context = __tw::response::Context::new(
                request,
                &self.inner.serializer);

            __tw::response::Response::into_http(response, &context).map(|response| {
                response.map(|body| ResponseBody(Err(body), ::std::marker::PhantomData))
            })
        })
    }

    /// The resource's future type
    fn handler_future_ty(&self) -> TokenStream {
        TyTree::new(&self.routes[..])
            .map_either(|route| route.future_ty())
    }

    fn extract_future_ty(&self) -> TokenStream {
        TyTree::new(&self.routes[..])
            .map_either(|route| route.handler_args_ty())
    }

    fn build_routes_fn(&self) -> TokenStream {
        self.destination_syms(|route, destination| {
            let destination = destination.build_default();
            route.build_route(destination)
        })
    }

    // Kind of a hack
    pub fn finalize(&mut self) {
        self.destinations.clear();

        self.destinations = TyTree::new(&self.routes[..])
            .map_reduce(
                |_| vec![Destination::new()],
                |destinations| {
                    let mut ret = vec![];
                    let len = destinations.len();

                    for (i, destinations) in destinations.iter().enumerate() {
                        let variant = variant(i, len);

                        for destination in destinations {
                            let mut destination = destination.clone();
                            destination.path.push(variant.clone());
                            ret.push(destination);
                        }
                    }

                    ret
                });
    }

    /// The token used as the resource destination.
    fn destination_syms<F, R>(&self, mut f: F) -> TokenStream
    where F: FnMut(&Route, &Destination) -> R,
          R: ::quote::ToTokens,
    {
        // TODO: Optimize
        let mut ret = TokenStream::new();

        for (i, route) in self.routes.iter().enumerate() {
            let destination = &self.destinations[i];

            ret.append_all(f(route, &destination).into_token_stream());
        }

        ret
    }
}

impl Destination {
    pub fn new() -> Destination {
        let path = vec![];
        Destination { path }
    }

    fn build_default(&self) -> TokenStream {
        self.build(quote!(()))
    }

    fn build<T: ToTokens>(&self, content: T) -> TokenStream {
        let mut ret = content.into_token_stream();

        for step in &self.path {
            ret = quote! { #step(#ret) };
        }

        ret
    }
}

fn variant(index: usize, max: usize) -> TokenStream {
    let either: syn::Type =
        syn::parse_str(&format!("__tw::util::tuple::Either{}", max)).unwrap();

    match index {
        0 => quote! { #either::A },
        1 => quote! { #either::B },
        2 => quote! { #either::C },
        3 => quote! { #either::D },
        4 => quote! { #either::E },
        5 => quote! { #either::F },
        6 => quote! { #either::G },
        7 => quote! { #either::H },
        8 => quote! { #either::I },
        9 => quote! { #either::J },
        10 => quote! { #either::K },
        11 => quote! { #either::L },
        n => panic!("unimplemented; variant {}", n),
    }
}

fn route_n(n: usize) -> Ident {
    Ident::new(
        &format!("route_{}", n),
        Span::call_site())
}

fn dummy_const_ident(ty: &syn::Type) -> String {
    use syn::Type::*;

    match *ty {
        Slice(ref ty) => format!("SLICE_{}", dummy_const_ident(&ty.elem)),
        Array(ref ty) => format!("ARRAY_{}", dummy_const_ident(&ty.elem)),
        Ptr(ref ty) => {
            let inner = dummy_const_ident(&ty.elem);

            if ty.const_token.is_some() {
                format!("CONST_PTR_{}", inner)
            } else if ty.mutability.is_some() {
                format!("MUT_PTR_{}", inner)
            } else {
                format!("PTR_{}", inner)
            }
        }
        Reference(ref ty) => {
            let inner = dummy_const_ident(&ty.elem);

            if ty.mutability.is_some() {
                format!("MUT_REF{}", inner)
            } else {
                format!("REF_{}", inner)
            }
        }
        BareFn(_) => unimplemented!(),
        Never(_) => unimplemented!(),
        Tuple(_) => unimplemented!(),
        Path(ref ty) => {
            ty.path.segments.iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join("_")
        }
        TraitObject(_) => unimplemented!(),
        ImplTrait(_) => unimplemented!(),
        Paren(_) => unimplemented!(),
        Group(_) => unimplemented!(),
        Infer(_) => unimplemented!(),
        Macro(_) => unimplemented!(),
        Verbatim(_) => unimplemented!(),
        _ => unimplemented!()
    }
}
