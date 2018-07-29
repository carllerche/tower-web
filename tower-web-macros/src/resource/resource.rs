use resource::Route;

use quote::{TokenStreamExt, ToTokens};
use proc_macro2::{Ident, Span, TokenStream};
use syn;

use std::cmp;

#[derive(Debug)]
pub(crate) struct Resource {
    index: usize,
    pub self_ty: Box<syn::Type>,
    pub routes: Vec<Route>,
}

impl Resource {
    pub fn new(index: usize, self_ty: Box<syn::Type>) -> Resource {
        Resource {
            index,
            self_ty,
            routes: vec![],
        }
    }

    /// Generate the implementation
    pub fn gen(&self) -> TokenStream {
        let resource_impl = if self.routes.is_empty() {
            self.gen_empty_impl()
        } else {
            self.gen_impl()
        };

        let dummy_const = self.dummy_const();
        let ty = &self.self_ty;

        quote! {
            #[allow(warnings)]
            const #dummy_const: () = {
                extern crate tower_web as __tw;

                #resource_impl

                impl<U> __tw::util::Chain<U> for #ty {
                    type Output = (Self, U);

                    fn chain(self, other: U) -> Self::Output {
                        (self, other)
                    }
                }
            };
        }
    }

    fn gen_impl(&self) -> TokenStream {
        let ty = &self.self_ty;

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

            pub struct GeneratedResource<S, B>
            where S: __tw::response::Serializer,
                  B: __tw::util::BufStream,
            {
                inner: ::std::sync::Arc<Inner<S>>,
                _p: ::std::marker::PhantomData<B>,
            }

            struct Inner<S>
            where S: __tw::response::Serializer,
            {
                handler: #ty,
                callsites: CallSites,
                content_types: ContentTypes<S>,
                serializer: S,
            }

            #callsites_def
            #content_types_def

            impl<S, B> GeneratedResource<S, B>
            where S: __tw::response::Serializer,
                  B: __tw::util::BufStream,
            {
                fn new(handler: #ty, serializer: S) -> Self {
                    let callsites = CallSites::new::<B>();
                    let content_types = ContentTypes::new(&serializer);

                    callsites.verify();

                    let inner = ::std::sync::Arc::new(Inner {
                        handler,
                        callsites,
                        content_types,
                        serializer,
                    });

                    GeneratedResource {
                        inner,
                        _p: ::std::marker::PhantomData,
                    }
                }
            }

            impl<S, B> Clone for GeneratedResource<S, B>
            where S: __tw::response::Serializer,
                  B: __tw::util::BufStream,
            {
                fn clone(&self) -> Self {
                    let inner = self.inner.clone();
                    GeneratedResource {
                        inner,
                        _p: ::std::marker::PhantomData,
                    }
                }
            }

            impl<S, B> __tw::service::IntoResource<S, B> for #ty
            where S: __tw::response::Serializer,
                  B: __tw::util::BufStream,
            {
                type Destination = #destination_ty;
                type Resource = GeneratedResource<S, B>;

                fn routes(&self) -> __tw::routing::RouteSet<Self::Destination> {
                    __tw::routing::Builder::new()
                    #build_routes_fn
                    .build()
                }

                fn into_resource(self, serializer: S) -> Self::Resource {
                    GeneratedResource::new(self, serializer)
                }
            }

            impl<S, B> __tw::service::Resource for GeneratedResource<S, B>
            where S: __tw::response::Serializer,
                  B: __tw::util::BufStream,
            {
                // The destination token is used to identify which action to
                // call
                type Destination = #destination_ty;

                type RequestBody = B;

                // The response body's chunk type.
                type Buf = <Self::Body as __tw::util::BufStream>::Item;

                // The reesponse body type
                type Body = <Self::Future as __tw::service::HttpResponseFuture>::Item;

                // Future representing processing the request.
                type Future = ResponseFuture<S, B>;

                fn dispatch(
                    &mut self,
                    destination: Self::Destination,
                    route_match: __tw::routing::RouteMatch,
                    body: Self::RequestBody,
                ) -> Self::Future
                {
                    let mut body = Some(body);
                    #dispatch_fn
                }
            }

            pub struct ResponseFuture<S, B>
            where S: __tw::response::Serializer,
                  B: __tw::util::BufStream,
            {
                state: State<B>,
                inner: ::std::sync::Arc<Inner<S>>,
                _p: ::std::marker::PhantomData<B>,
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
            enum State<B>
            where B: __tw::util::BufStream,
            {
                Extract(#extract_future_ty),
                Response(#handler_future_ty),
                // PhantomData is needed for when the resource has no routes
                // that use the body component.
                Invalid(::std::marker::PhantomData<B>),
            }

            impl<S, B> __tw::codegen::futures::Future for ResponseFuture<S, B>
            where S: __tw::response::Serializer,
                  B: __tw::util::BufStream,
            {
                type Item = __tw::codegen::http::Response<ResponseBody>;
                type Error = __tw::Error;

                fn poll(&mut self) -> __tw::codegen::futures::Poll<Self::Item, Self::Error> {
                    loop {
                        match self.state {
                            State::Extract(ref mut extract_future) => {
                                try_ready!(__tw::codegen::futures::Future::poll(extract_future));
                            }
                            State::Response(ref mut response) => {
                                return Ok(__tw::codegen::futures::Async::Ready(match try_ready!(response.poll()) {
                                    #match_into_response
                                }));
                            }
                            State::Invalid(_) => unreachable!(),
                        }

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

            /// Response body
            pub struct ResponseBody(<<#handler_future_ty as __tw::codegen::futures::Future>::Item as __tw::response::Response>::Body);

            /// Response buf
            pub struct ResponseBuf(<<#handler_future_ty as __tw::codegen::futures::Future>::Item as __tw::response::Response>::Buf);

            impl __tw::util::BufStream for ResponseBody {
                type Item = ResponseBuf;
                type Error = __tw::Error;

                fn poll(&mut self) -> __tw::codegen::futures::Poll<Option<Self::Item>, Self::Error> {
                    let buf = try_ready!(self.0.poll());
                    Ok(buf.map(ResponseBuf).into())
                }

                fn size_hint(&self) -> __tw::util::buf_stream::SizeHint {
                    self.0.size_hint()
                }
            }

            impl ::std::fmt::Debug for ResponseBody {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    fmt.debug_struct("ResponseBody")
                        .finish()
                }
            }

            // TODO: Implement default fns
            impl __tw::codegen::bytes::Buf for ResponseBuf {
                fn remaining(&self) -> usize {
                    self.0.remaining()
                }

                fn bytes(&self) -> &[u8] {
                    self.0.bytes()
                }

                fn advance(&mut self, cnt: usize) {
                    self.0.advance(cnt)
                }
            }

            impl ::std::fmt::Debug for ResponseBuf {
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

        quote! {
            impl<S, B> __tw::service::IntoResource<S, B> for #ty
            where S: __tw::response::Serializer,
                  B: __tw::util::BufStream,
            {
                type Destination = ();
                type Resource = __tw::service::Unit<B>;

                fn routes(&self) -> __tw::routing::RouteSet<Self::Destination> {
                    __tw::routing::RouteSet::new()
                }

                fn into_resource(self, serializer: S) -> Self::Resource {
                    __tw::service::Unit::new()
                }
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
        let mut ret = quote! { () };
        let mut rem = self.routes.len();
        let mut level = 0;

        if rem == 0 {
            return quote!(());
        }

        while rem > 0 {
            let mut max = ::MAX_VARIANTS;

            if level > 0 {
                max -= 1;
            }

            let mut variants = cmp::min(rem, max);

            rem -= variants;

            if level > 0 {
                variants += 1;
            }

            level += 1;

            ret = match variants {
                1 => quote! { __tw::util::tuple::Either1<#ret> },
                2 => quote! { __tw::util::tuple::Either2<#ret, ()> },
                3 => quote! { __tw::util::tuple::Either3<#ret, (), ()> },
                n => panic!("unimplemented: {} variants Service::destination_ty", n),
            };
        }

        ret
    }

    fn callsites_def(&self) -> TokenStream {
        let fields = self.routes.iter().enumerate()
            .map(|(i, route)| {
                let name = route_n(i);
                let tys = (0..route.args.len())
                    .map(|_| quote!((__tw::codegen::CallSite, bool),));

                quote! { #name: (#(#tys)*) }
            });

        let init = self.routes.iter().enumerate()
            .map(|(i, route)| {
                let name = route_n(i);
                let init = route.args.iter()
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
                let verify = route.args.iter()
                    .map(|arg| {
                        let index = arg.index;

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
                if let Some(ref content_type) = route.rules.content_type {
                    quote!(Some(serializer.lookup(#content_type).unwrap_or_else(|| panic!("unsupported format '{}'", #content_type))))
                } else {
                    quote!(None)
                }
            });

        quote! {
            struct ContentTypes<S: __tw::response::Serializer> {
                content_types: [Option<S::ContentType>; #num],
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
                #(#branches)*
            };

            let state = State::Extract(either);
            let inner = self.inner.clone();

            ResponseFuture {
                state,
                inner,
                _p: ::std::marker::PhantomData,
            }
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
        self.destination_syms(|route, destination| {
            let left = destination.build(quote!(response));
            let map = destination.build(quote!(body));
            let idx = route.index;

            quote! {
                #left => {
                    let content_type = self.inner.content_types
                        .content_types[#idx]
                        .as_ref()
                        .expect("no content type specified");

                    let context = __tw::response::Context::new(
                        &self.inner.serializer,
                        content_type);

                    __tw::response::Response::into_http(response, &context)
                        .map(|body| ResponseBody(#map))
                }
            }
        })
    }

    /// The resource's future type
    fn handler_future_ty(&self) -> TokenStream {
        self.routes_ty(|route| {
            let ty = &route.ret;

            if route.box_ret {
                quote! { #ty }
            } else {
                quote! {
                    __tw::response::MapErr<<#ty as __tw::codegen::futures::IntoFuture>::Future>
                }
            }
        })
    }

    fn extract_future_ty(&self) -> TokenStream {
        self.routes_ty(|route| route.handler_args_ty())
    }

    fn build_routes_fn(&self) -> TokenStream {
        self.destination_syms(|route, destination| {
            let destination = destination.build_default();
            route.build_route(destination)
        })
    }

    /// Generates an `Either` type with a variant per route.
    fn routes_ty<F, R>(&self, mut f: F) -> TokenStream
    where F: FnMut(&Route) -> R,
          R: ::quote::ToTokens,
    {
        let mut routes = &self.routes[..];

        assert!(!self.routes.is_empty());

        let mut ret = quote! { () };
        let mut level = 0;

        while !routes.is_empty() {
            let mut max = ::MAX_VARIANTS;

            if level > 0 {
                max -= 1;
            }

            let num = cmp::min(routes.len(), max);

            let mut variant_tys = vec![];

            if level > 0 {
                variant_tys.push(ret);
            }

            // Get the types of each variant
            variant_tys.extend({
                routes[..num].iter()
                    .map(&mut f)
                    .map(|ret| ret.into_token_stream())
            });

            let either: syn::Type = syn::parse_str(&format!(
                "__tw::util::tuple::Either{}",
                variant_tys.len()
            )).unwrap();

            ret = quote! {
                #either<#(#variant_tys),*>
            };

            routes = &routes[num..];
            level += 1;
        }

        ret
    }

    /// The token used as the resource destination.
    fn destination_syms<F, R>(&self, mut f: F) -> TokenStream
    where F: FnMut(&Route, &Destination) -> R,
          R: ::quote::ToTokens,
    {
        // TODO: Optimize
        let mut ret = TokenStream::new();

        for (i, route) in self.routes.iter().enumerate() {
            let destination = Destination {
                index: i,
                total: self.routes.len(),
            };

            ret.append_all(f(route, &destination).into_token_stream());
        }

        ret
    }
}

struct Destination {
    index: usize,
    total: usize,
}

impl Destination {
    fn build_default(&self) -> TokenStream {
        self.build(quote!(()))
    }

    fn build<T: ToTokens>(&self, content: T) -> TokenStream {
        let mut content = Some(content.into_token_stream());
        let mut index = self.index;
        let mut total = self.total;
        let mut ret = None;
        let mut level = 0;
        let mut max = ::MAX_VARIANTS;

        assert!(total > 0);

        while total > 0 {
            if ret.is_none() {
                if index < max {
                    let mut i = index;
                    let mut n = cmp::min(total, max);

                    if level > 0 {
                        i += 1;
                        n += 1;
                    }

                    let content = content.take().unwrap();

                    ret = Some(variant(i, n, content));
                } else {
                    index -= max;
                }
            } else {
                let n = cmp::min(total, max);
                let v = ret.take().unwrap();
                ret = Some(variant(0, n + 1, v));
            }

            total = total.saturating_sub(max);
            level += 1;

            if max == ::MAX_VARIANTS {
                max -= 1;
            }
        }

        ret.unwrap()
    }
}

fn variant(index: usize, max: usize, content: TokenStream) -> TokenStream {
    let either: syn::Type =
        syn::parse_str(&format!("__tw::util::tuple::Either{}", max)).unwrap();

    match index {
        0 => quote! { #either::A(#content) },
        1 => quote! { #either::B(#content) },
        2 => quote! { #either::C(#content) },
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
    }
}
