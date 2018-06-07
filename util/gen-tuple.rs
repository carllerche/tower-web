
const VARS: &[&str] = &[
    "A",
    "B",
    "C",
    "D",
    "E",
    "F",
    "G",
    "H",
    "I",
    "J",
    /*
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

use super::{Chain, Resource};
use routing::{self, RouteSet, RouteMatch};

use bytes::Bytes;
use futures::{Future, Poll};
use futures::future::FutureResult;
use futures::stream::Once;
use http;

// ===== 0 =====

impl Resource for () {
    type Destination = ();
    type Body = Once<Bytes, ::Error>;
    type Future = FutureResult<http::Response<Self::Body>, ::Error>;

    fn routes(&self) -> RouteSet<()> {
        RouteSet::default()
    }

    fn dispatch(&mut self, _: (), _: RouteMatch, _: http::Request<()>) -> Self::Future {
        unreachable!();
    }
}

impl<U> Chain<U> for () {
    type Resource = U;

    fn chain(self, other: U) -> Self::Resource {
        other
    }
}"##[1..]);

    for i in 0..(VARS.len() - 1) {
        gen_either(i);
    }
}

fn gen_either(i: usize) {

    let variants = 2 + i;

    let gens = VARS[0..variants].iter()
        .map(|ty| format!("{} = ()", ty))
        .collect::<Vec<_>>()
        .join(", ");

    println!("// ===== {} =====", variants);
    println!("");
    println!("#[derive(Clone)]");
    println!("pub enum Either{}<{}> {{", variants, gens);

    for n in 0..variants {
        println!("    {}({}),", VARS[n], VARS[n]);
    }

    println!("}}");
    println!("");

    let gens = VARS[0..variants].iter()
        .map(|ty| format!("{}", ty))
        .collect::<Vec<_>>()
        .join(", ");

    println!("impl<{}> Future for Either{}<{}>", gens, variants, gens);
    println!("where");

    for n in 0..variants {
        if n == 0 {
            println!("    {}: Future,", VARS[n]);
        } else {
            println!("    {}: Future<Item = A::Item, Error = A::Error>,", VARS[n]);
        }
    }

    println!("{{");
    println!("    type Item = A::Item;");
    println!("    type Error = A::Error;");
    println!("");
    println!("    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {{");
    println!("        use self::Either{}::*;", variants);
    println!("");
    println!("        match *self {{");

    for n in 0..variants {
        println!("            {}(ref mut f) => f.poll(),", VARS[n]);
    }

    println!("        }}");
    println!("    }}");
    println!("}}");
    println!("");

    let gens = (0..variants)
        .map(|ty| format!("R{}", ty))
        .collect::<Vec<_>>()
        .join(", ");

    println!("impl<{}> Resource for ({})", gens, gens);
    println!("where");


    for n in 0..variants {
        if n == 0 {
            println!("    R{}: Resource,", n);
        } else {
            println!("    R{}: Resource<Body = R0::Body>,", n);
        }
    }

    println!("{{");

    let gens = (0..variants)
        .map(|ty| format!("R{}::Destination", ty))
        .collect::<Vec<_>>()
        .join(", ");

    println!("    type Destination = Either{}<{}>;", variants, gens);
    println!("    type Body = R0::Body;");

    let gens = (0..variants)
        .map(|ty| format!("R{}::Future", ty))
        .collect::<Vec<_>>()
        .join(", ");

    println!("    type Future = Either{}<{}>;", variants, gens);
    println!("");
    println!("    fn routes(&self) -> RouteSet<Self::Destination> {{");
    println!("        let mut routes = routing::Builder::new();");
    println!("");

    for n in 0..variants {
        println!("        for route in self.{}.routes() {{", n);
        println!("            routes.push(route.map(Either{}::{}));", variants, VARS[n]);
        println!("        }}");
        println!("");
    }

    println!("        routes.build()");
    println!("    }}");
    println!("");
    println!("    fn dispatch(&mut self,");
    println!("                destination: Self::Destination,");
    println!("                route_match: RouteMatch,");
    println!("                request: http::Request<()>)");
    println!("        -> Self::Future");
    println!("    {{");
    println!("        use self::Either{}::*;", variants);
    println!("");
    println!("        match destination {{");

    for n in 0..variants {
        println!("            {}(d) => {{", VARS[n]);
        println!("                {}(self.{}.dispatch(d, route_match, request))", VARS[n], n);
        println!("            }}");
    }

    println!("        }}");
    println!("    }}");
    println!("}}");
    println!("");

    let gens = (0..variants)
        .map(|ty| format!("R{}", ty))
        .collect::<Vec<_>>()
        .join(", ");

    println!("impl<{}, U> Chain<U> for ({}) {{", gens, gens);
    println!("    type Resource = ({}, U);", gens);
    println!("");
    println!("    fn chain(self, other: U) -> Self::Resource {{");

    let vals = (0..variants)
        .map(|ty| format!("self.{}", ty))
        .collect::<Vec<_>>()
        .join(", ");

    println!("        ({}, other)", vals);
    println!("    }}");
    println!("}}");
}
