
const VARS: &[&str] = &[
    "A",
    "B",
    "C",
    /*
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "Q",
    "R",
    "S",
    "T",
    "U",
    "V",
    "W",
    "X",
    "Y",
    "Z",
    */
];

pub fn main() {
    println!("{}", &r##"
//! Implementations of `Resource` for tuple types.

// NOTE: This file should not be updated directly. Instead, update
// `util/gen-tuple.rs` and regenerate this file.

use extract::{self, ExtractFuture};
use response::{Context, IntoResponse, MapErr, Serializer};
use routing::{self, RouteSet, RouteMatch};
use service::{Resource, IntoResource, HttpResponseFuture};
use util::{BufStream, Chain};

use bytes::Buf;
use futures::{Future, Stream, Async, Poll};
use futures::future::FutureResult;
use http;

// ===== Utility traits =====

pub struct LiftHttpResponse<T> {
    inner: T,
}

impl<T: HttpResponseFuture> Future for LiftHttpResponse<T> {
    type Item = http::Response<T::Item>;
    type Error = ::Error;

    fn poll(&mut self) -> Poll<Self::Item, ::Error> {
        self.inner.poll_http_response()
    }
}

// ===== 0 =====

impl Resource for () {
    type Destination = ();
    type Buf = <Self::Body as BufStream>::Item;
    type Body = MapErr<String>;
    type Future = FutureResult<http::Response<Self::Body>, ::Error>;

    fn dispatch<In: BufStream>(&mut self, _: (), _: RouteMatch, _: In) -> Self::Future {
        unreachable!();
    }
}

impl<S: Serializer> IntoResource<S> for () {
    type Destination = ();
    type Resource = ();

    fn routes(&self) -> RouteSet<()> {
        RouteSet::new()
    }

    fn into_resource(self, _: S) -> Self::Resource {
        ()
    }
}

pub struct Join0 {
    _p: (),
}

impl Join0 {
    pub fn new() -> Self {
        Self { _p: () }
    }

    pub fn into_inner(self) -> () {
        ()
    }
}

impl Future for Join0 {
    type Item = ();
    type Error = extract::Error;

    fn poll(&mut self) -> Poll<(), extract::Error> {
        Ok(().into())
    }
}

impl<U> Chain<U> for () {
    type Output = U;

    fn chain(self, other: U) -> Self::Output {
        other
    }
}"##[1..]);

    Either::gen_all();

    println!("");

    Tuple::gen_all();
}

struct Either {
    level: usize,
}

impl Either {
    fn gen_all() {
        if VARS.is_empty() {
            return;
        }

        for i in 0..VARS.len() {
            let either = Either::new(1 + i);
            either.gen();
        }
    }

    fn new(level: usize) -> Either {
        Either {
            level,
        }
    }

    fn gen(&self) {
        let gens = VARS[0..self.level].iter()
            .map(|ty| format!("{}", ty))
            .collect::<Vec<_>>()
            .join(", ");

        let item_gens = VARS[0..self.level].iter()
            .map(|ty| format!("{}::Item", ty))
            .collect::<Vec<_>>()
            .join(", ");

        let buf_gens = VARS[0..self.level].iter()
            .map(|ty| format!("{}::Buf", ty))
            .collect::<Vec<_>>()
            .join(", ");

        let body_gens = VARS[0..self.level].iter()
            .map(|ty| format!("{}::Body", ty))
            .collect::<Vec<_>>()
            .join(", ");

        println!("// ===== {} =====", self.level);
        println!("");

        println!("#[derive(Clone)]");
        println!("pub enum Either{}<{}> {{", self.level, gens);

        for n in 0..self.level {
            println!("    {}({}),", VARS[n], VARS[n]);
        }

        println!("}}");
        println!("");

        // ===== impl Future or Either =====

        println!("impl<{}> Future for Either{}<{}>", gens, self.level, gens);
        println!("where");

        for n in 0..self.level {
            if n == 0 {
                println!("    {}: Future,", VARS[n]);
            } else {
                println!("    {}: Future<Error = A::Error>,", VARS[n]);
            }
        }

        println!("{{");
        println!("    type Item = Either{}<{}>;", self.level, item_gens);
        println!("    type Error = A::Error;");
        println!("");
        println!("    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {{");
        println!("        use self::Either{}::*;", self.level);
        println!("");
        println!("        match *self {{");

        for n in 0..self.level {
            println!("            {}(ref mut f) => Ok({}(try_ready!(f.poll())).into()),", VARS[n], VARS[n]);
        }

        println!("        }}");
        println!("    }}");
        println!("}}");
        println!("");

        // ===== impl Either =====

        println!("impl<{}> Either{}<{}>", gens, self.level, gens);
        println!("where");

        for n in 0..self.level {
            println!("    {}: ExtractFuture,", VARS[n]);
        }

        println!("{{");
        println!("");
        println!("    pub fn poll_ready(&mut self) -> Poll<(), extract::Error> {{");
        println!("        use self::Either{}::*;", self.level);
        println!("");
        println!("        match *self {{");

        for n in 0..self.level {
            println!("            {}(ref mut f) => f.poll(),", VARS[n]);
        }

        println!("        }}");
        println!("    }}");
        println!("}}");
        println!("");

        // ===== impl HttpResponseFuture =====

        println!("impl<{}> HttpResponseFuture for Either{}<{}>", gens, self.level, gens);
        println!("where");

        for n in 0..self.level {
            println!("    {}: HttpResponseFuture,", VARS[n]);
        }

        println!("{{");
        println!("    type Item = Either{}<{}>;", self.level, item_gens);
        println!("");
        println!("    fn poll_http_response(&mut self) -> Poll<http::Response<Self::Item>, ::Error> {{");
        println!("        use self::Either{}::*;", self.level);
        println!("");
        println!("        match *self {{");

        for n in 0..self.level {
            println!("            {}(ref mut f) => Ok(try_ready!(f.poll_http_response()).map({}).into()),", VARS[n], VARS[n]);
        }

        println!("        }}");
        println!("    }}");
        println!("}}");
        println!("");

        // ===== impl Stream or Either =====

        println!("impl<{}> Stream for Either{}<{}>", gens, self.level, gens);
        println!("where");

        for n in 0..self.level {
            if n == 0 {
                println!("    {}: Stream,", VARS[n]);
            } else {
                println!("    {}: Stream<Error = A::Error>,", VARS[n]);
            }
        }

        println!("{{");
        println!("    type Item = Either{}<{}>;", self.level, item_gens);
        println!("    type Error = A::Error;");
        println!("");
        println!("    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {{");
        println!("        use self::Either{}::*;", self.level);
        println!("");
        println!("        match *self {{");

        for n in 0..self.level {
            println!("            {}(ref mut f) => Ok(try_ready!(f.poll()).map({}).into()),", VARS[n], VARS[n]);
        }

        println!("        }}");
        println!("    }}");
        println!("}}");
        println!("");

        // ===== impl BufStream or Either =====

        println!("impl<{}> BufStream for Either{}<{}>", gens, self.level, gens);
        println!("where");

        for n in 0..self.level {
            if n == 0 {
                println!("    {}: BufStream,", VARS[n]);
            } else {
                println!("    {}: BufStream<Error = A::Error>,", VARS[n]);
            }
        }

        println!("{{");
        println!("    type Item = Either{}<{}>;", self.level, item_gens);
        println!("    type Error = A::Error;");
        println!("");
        println!("    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {{");
        println!("        use self::Either{}::*;", self.level);
        println!("");
        println!("        match *self {{");

        for n in 0..self.level {
            println!("            {}(ref mut f) => Ok(try_ready!(f.poll()).map({}).into()),", VARS[n], VARS[n]);
        }

        println!("        }}");
        println!("    }}");
        println!("}}");
        println!("");

        // ===== impl Buf or Either =====

        println!("impl<{}> Buf for Either{}<{}>", gens, self.level, gens);
        println!("where");
        for n in 0..self.level {
            println!("    {}: Buf,", VARS[n]);
        }
        println!("{{");
        println!("    fn remaining(&self) -> usize {{");
        println!("        use self::Either{}::*;", self.level);
        println!("");
        println!("        match *self {{");
        for n in 0..self.level {
            println!("            {}(ref b) => b.remaining(),", VARS[n]);
        }
        println!("        }}");
        println!("    }}");
        println!("");
        println!("    fn bytes(&self) -> &[u8] {{");
        println!("        use self::Either{}::*;", self.level);
        println!("");
        println!("        match *self {{");
        for n in 0..self.level {
            println!("            {}(ref b) => b.bytes(),", VARS[n]);
        }
        println!("        }}");
        println!("    }}");
        println!("");
        println!("    fn advance(&mut self, cnt: usize) {{");
        println!("        use self::Either{}::*;", self.level);
        println!("");
        println!("        match *self {{");
        for n in 0..self.level {
            println!("            {}(ref mut b) => b.advance(cnt),", VARS[n]);
        }
        println!("        }}");
        println!("    }}");
        println!("}}");
        println!("");

        // ===== impl IntoResponse or Either =====

        println!("impl<{}> IntoResponse for Either{}<{}>", gens, self.level, gens);
        println!("where");
        for n in 0..self.level {
            println!("    {}: IntoResponse,", VARS[n]);
        }
        println!("{{");
        println!("    type Buf = Either{}<{}>;", self.level, buf_gens);
        println!("    type Body = Either{}<{}>;", self.level, body_gens);
        println!("");
        println!("    fn into_response<S>(self, context: &Context<S>) ->  http::Response<Self::Body>");
        println!("    where S: Serializer");
        println!("    {{");
        println!("        use self::Either{}::*;", self.level);
        println!("");
        println!("        match self {{");
        for n in 0..self.level {
            println!("            {}(r) => r.into_response(context).map(Either{}::{}),", VARS[n], self.level, VARS[n]);
        }
        println!("        }}");
        println!("    }}");
        println!("}}");
        println!("");

        let gens = (0..self.level)
            .map(|ty| format!("R{}", ty))
            .collect::<Vec<_>>()
            .join(", ");

        // ===== impl Resource for (...) =====

        println!("impl<{}> Resource for ({},)", gens, gens);
        println!("where");


        for n in 0..self.level {
            println!("    R{}: Resource,", n);
        }

        println!("{{");

        let gens = (0..self.level)
            .map(|ty| format!("R{}::Destination", ty))
            .collect::<Vec<_>>()
            .join(", ");

        println!("    type Destination = Either{}<{}>;", self.level, gens);

        let gens = (0..self.level)
            .map(|ty| format!("R{}::Buf", ty))
            .collect::<Vec<_>>()
            .join(", ");

        println!("    type Buf = Either{}<{}>;", self.level, gens);

        let gens = (0..self.level)
            .map(|ty| format!("R{}::Body", ty))
            .collect::<Vec<_>>()
            .join(", ");

        println!("    type Body = Either{}<{}>;", self.level, gens);

        let gens = (0..self.level)
            .map(|ty| format!("R{}::Future", ty))
            .collect::<Vec<_>>()
            .join(", ");

        println!("    type Future = LiftHttpResponse<Either{}<{}>>;", self.level, gens);
        println!("");
        println!("    fn dispatch<In: BufStream>(&mut self,");
        println!("                               destination: Self::Destination,");
        println!("                               route_match: RouteMatch,");
        println!("                               body: In)");
        println!("        -> Self::Future");
        println!("    {{");
        println!("        use self::Either{}::*;", self.level);
        println!("");
        println!("        let inner = match destination {{");

        for n in 0..self.level {
            println!("            {}(d) => {{", VARS[n]);
            println!("                {}(self.{}.dispatch(d, route_match, body))", VARS[n], n);
            println!("            }}");
        }

        println!("        }};");
        println!("");
        println!("        LiftHttpResponse {{ inner }}");
        println!("    }}");
        println!("}}");
    }
}

struct Tuple {
    level: usize,
}

impl Tuple {
    fn gen_all() {
        for i in 0..2 {
            let tuple = Tuple::new(1 + i);
            tuple.gen();
        }
    }

    fn new(level: usize) -> Tuple {
        Tuple {
            level,
        }
    }

    fn gen(&self) {
        let gens = (0..self.level)
            .map(|ty| format!("R{}", ty))
            .collect::<Vec<_>>()
            .join(", ");

        println!("impl<{}, U> Chain<U> for ({},) {{", gens, gens);
        println!("    type Output = ({}, U);", gens);
        println!("");
        println!("    fn chain(self, other: U) -> Self::Output {{");

        let vals = (0..self.level)
            .map(|ty| format!("self.{}", ty))
            .collect::<Vec<_>>()
            .join(", ");

        println!("        ({}, other)", vals);
        println!("    }}");
        println!("}}");
        println!("");

        let args = (0..self.level)
            .map(|ty| format!("t{}: T{}", ty, ty))
            .collect::<Vec<_>>()
            .join(", ");

        let pending = (0..self.level)
            .map(|_| "false, ")
            .collect::<Vec<_>>()
            .join("");

        let futures = (0..self.level)
            .map(|ty| format!("t{}, ", ty))
            .collect::<Vec<_>>()
            .join("");

        let gens = (0..self.level)
            .map(|ty| format!("T{}", ty))
            .collect::<Vec<_>>()
            .join(", ");

        let destinations = (0..self.level)
            .map(|ty| format!("T{}::Destination", ty))
            .collect::<Vec<_>>()
            .join(", ");

        let resources = (0..self.level)
            .map(|ty| format!("T{}::Resource", ty))
            .collect::<Vec<_>>()
            .join(", ");

        let bools = (0..self.level)
            .map(|_| "bool")
            .collect::<Vec<_>>()
            .join(", ");

        println!("pub struct Join{}<{}> {{", self.level, gens);
        println!("    futures: ({},),", gens);
        println!("    pending: ({},),", bools);
        println!("}}");
        println!("");
        println!("impl<{}> Join{}<{}>", gens, self.level, gens);
        println!("where");
        for i in 0..self.level {
            println!("    T{}: ExtractFuture,", i);
        }
        println!("{{");
        println!("    pub fn new({}) -> Self {{", args);
        println!("        Self {{");
        println!("            pending: ({}),", pending);
        println!("            futures: ({}),", futures);
        println!("        }}");
        println!("    }}");
        println!("");
        println!("    pub fn into_inner(self) -> ({},)", gens);
        println!("    {{");
        println!("        self.futures");
        println!("    }}");
        println!("}}");
        println!("");
        println!("impl<{}> Future for Join{}<{}>", gens, self.level, gens);
        println!("where");
        for i in 0..self.level {
            println!("    T{}: ExtractFuture,", i);
        }
        println!("{{");
        println!("    type Item = ();");
        println!("    type Error = extract::Error;");
        println!("");
        println!("    fn poll(&mut self) -> Poll<(), extract::Error> {{");
        println!("    let mut all_ready = true;");
        println!("");
        for i in 0..self.level {
            println!("        if !self.pending.{} {{", i);
            println!("            self.pending.{} = self.futures.{}.poll()?.is_ready();", i, i);
            println!("            all_ready &= self.pending.{};", i);
            println!("        }}");
        }
        println!("        Ok(if all_ready {{ Async::Ready(()) }} else {{ Async::NotReady }})");
        println!("    }}");
        println!("}}");

        // ===== impl IntoResource for (...) =====

        println!("impl<S: Serializer, {}> IntoResource<S> for ({},)", gens, gens);
        println!("where");

        for n in 0..self.level {
            println!("    T{}: IntoResource<S>,", n);
        }

        println!("{{");
        println!("    type Destination = Either{}<{}>;", self.level, destinations);
        println!("    type Resource = ({},);", resources);
        println!("");
        println!("    fn routes(&self) -> RouteSet<Self::Destination>");
        println!("    {{");
        println!("        let mut routes = routing::Builder::new();");
        println!("");

        for n in 0..self.level {
            println!("        for route in self.{}.routes() {{", n);
            println!("            routes.push(route.map(Either{}::{}));", self.level, VARS[n]);
            println!("        }}");
            println!("");
        }

        println!("        routes.build()");
        println!("    }}");
        println!("");
        println!("    fn into_resource(self, serializer: S) -> Self::Resource {{");
        println!("        (");
        for i in 0..self.level {
            println!("            self.{}.into_resource(serializer.clone()),", i);
        }
        println!("        )");
        println!("    }}");
        println!("}}");
    }
}
