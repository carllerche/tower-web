
const VARS: &[&str] = &[
    "A",
    "B",
    /*
    "C",
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

use Payload;
use super::{Chain, Resource};
use response::IntoResponse;
use routing::{self, RouteSet, RouteMatch};

use bytes::{Bytes, Buf};
use futures::{Future, Stream, Poll};
use futures::future::FutureResult;
use futures::stream::Once;
use http;

use std::io;

// ===== 0 =====

impl Resource for () {
    type Destination = ();
    type Buf = io::Cursor<Bytes>;
    type Body = Once<Self::Buf, ::Error>;
    type Response = String;
    type Future = FutureResult<String, ::Error>;

    fn routes(&self) -> RouteSet<()> {
        RouteSet::default()
    }

    fn dispatch<T: Payload>(&mut self, _: (), _: &RouteMatch, _: &http::Request<()>, _: T) -> Self::Future {
        unreachable!();
    }
}

impl<U> Chain<U> for () {
    type Resource = U;

    fn chain(self, other: U) -> Self::Resource {
        other
    }
}"##[1..]);

    Either::gen_all();
}

struct Either {
    level: usize,
}

impl Either {
    fn gen_all() {
        if VARS.is_empty() {
            return;
        }

        for i in 0..(VARS.len() - 1) {
            let either = Either::new(2 + i);
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
            println!("            {}(ref mut f) => Ok(Either{}::{}(try_ready!(f.poll())).into()),", VARS[n], self.level, VARS[n]);
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
            println!("            {}(ref mut f) => Ok(try_ready!(f.poll()).map(Either{}::{}).into()),", VARS[n], self.level, VARS[n]);
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

        // ===== impl Response or Either =====

        println!("impl<{}> IntoResponse for Either{}<{}>", gens, self.level, gens);
        println!("where");
        for n in 0..self.level {
            println!("    {}: IntoResponse,", VARS[n]);
        }
        println!("{{");
        println!("    type Buf = Either{}<{}>;", self.level, buf_gens);
        println!("    type Body = Either{}<{}>;", self.level, body_gens);
        println!("");
        println!("    fn into_response(self) ->  http::Response<Self::Body> {{");
        println!("        use self::Either{}::*;", self.level);
        println!("");
        println!("        match self {{");
        for n in 0..self.level {
            println!("            {}(r) => r.into_response().map(Either{}::{}),", VARS[n], self.level, VARS[n]);
        }
        println!("        }}");
        println!("    }}");
        println!("}}");
        println!("");

        let gens = (0..self.level)
            .map(|ty| format!("R{}", ty))
            .collect::<Vec<_>>()
            .join(", ");

        println!("impl<{}> Resource for ({})", gens, gens);
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
            .map(|ty| format!("R{}::Response", ty))
            .collect::<Vec<_>>()
            .join(", ");

        println!("    type Response = Either{}<{}>;", self.level, gens);

        let gens = (0..self.level)
            .map(|ty| format!("R{}::Future", ty))
            .collect::<Vec<_>>()
            .join(", ");

        println!("    type Future = Either{}<{}>;", self.level, gens);
        println!("");
        println!("    fn routes(&self) -> RouteSet<Self::Destination> {{");
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
        println!("    fn dispatch<T: Payload>(&mut self,");
        println!("                            destination: Self::Destination,");
        println!("                            route_match: &RouteMatch,");
        println!("                            request: &http::Request<()>,");
        println!("                            payload: T,)");
        println!("        -> Self::Future");
        println!("    {{");
        println!("        use self::Either{}::*;", self.level);
        println!("");
        println!("        match destination {{");

        for n in 0..self.level {
            println!("            {}(d) => {{", VARS[n]);
            println!("                {}(self.{}.dispatch(d, route_match, request, payload))", VARS[n], n);
            println!("            }}");
        }

        println!("        }}");
        println!("    }}");
        println!("}}");
        println!("");

        let gens = (0..self.level)
            .map(|ty| format!("R{}", ty))
            .collect::<Vec<_>>()
            .join(", ");

        println!("impl<{}, U> Chain<U> for ({}) {{", gens, gens);
        println!("    type Resource = ({}, U);", gens);
        println!("");
        println!("    fn chain(self, other: U) -> Self::Resource {{");

        let vals = (0..self.level)
            .map(|ty| format!("self.{}", ty))
            .collect::<Vec<_>>()
            .join(", ");

        println!("        ({}, other)", vals);
        println!("    }}");
        println!("}}");
    }
}
