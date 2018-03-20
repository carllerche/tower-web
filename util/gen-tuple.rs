
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

use super::{Chain, Resource};
use routing::{RouteSet, Match};

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
        RouteSet::new()
    }

    fn dispatch(&mut self, _: Match<()>, _: http::Request<()>) -> Self::Future {
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
    println!("#[derive(Clone)");
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
    println!("    type Destination = Either");
    println!("    type Body = R0::Body");
    println!("    type Future = Either");
    println!("");
    println!("    fn routes(&self) -> RouteSet<Self::Destination> {{");
    println!("        let mut routes = RouteSet::new();");
    println!("");

    for n in 0..variants {
        println!("        for route in self.{}.routes() {{", n);
        println!("            routes.push(route.map(Either{}::{}));", variants, VARS[n]);
        println!("        }}");
        println!("");
    }

    println!("        routes");
    println!("    }}");
    println!("");
    println!("    fn dispatch(&mut self,");
    println!("                match_: Match<Self::Destination>,");
    println!("                request: http::Request<()>)");
    println!("        -> Self::Future");
    println!("    {{");
    println!("        use self::Either{}::*;", variants);
    println!("");
    println!("        let (destination, condition = match_.into_parts();");
    println!("");
    println!("        match destination {{");

    for n in 0..variants {
        println!("            {}(d) => {{", VARS[n]);
        println!("                let match_ = Match::new(d, condition);");
        println!("                {}(self.{}.dispatch(match_, request))", VARS[n], n);
        println!("            }}");
    }

    println!("        }}");
    /*
        use self::Either2::*;

        let (destination, condition) = match_.into_parts();

        match destination {
            A(d) => {
                let match_ = Match::new(d, condition);
                A(self.0.dispatch(match_, request))
            }
            B(d) => {
                let match_ = Match::new(d, condition);
                B(self.1.dispatch(match_, request))
            }
        }
     */
    println!("    }}");
    println!("");
    println!("}}");
    println!("");
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
    println!("");
    println!("}}");


    /*
impl<R1, R2> Resource for (R1, R2)
where R1: Resource,
      R2: Resource<Body = R1::Body>,
{
    type Destination = Either2<R1::Destination, R2::Destination>;
    type Body = R1::Body;
    type Future = Either2<R1::Future, R2::Future>;

    fn routes(&self) -> RouteSet<Self::Destination> {
        let mut routes = RouteSet::new();

        for route in self.0.routes() {
            routes.push(route.map(Either2::A));
        }

        for route in self.1.routes() {
            routes.push(route.map(Either2::B));
        }

        routes
    }

    fn dispatch(&mut self,
                match_: Match<Self::Destination>,
                request: http::Request<()>)
        -> Self::Future
    {

    }
}

impl<R1, R2, U> Chain<U> for (R1, R2) {
    type Resource = (R1, R2, U);

    fn chain(self, other: U) -> Self::Resource {
        (self.0, self.1, other)
    }
}"#);
*/

}
