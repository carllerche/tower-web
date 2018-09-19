/// Web service that receives and responds with HTML.
///
/// ## Overview
///
/// TODO: Dox
///
/// ## Usage
///
/// Run the example:
///
///     cargo run --example html_handlebars
///
/// Then send a request:
///
///     curl -v http://localhost:8080/

#[macro_use]
extern crate tower_web;
extern crate tokio;

#[macro_use]
extern crate serde_json;

use tower_web::ServiceBuilder;

/// This type will be the web service implementation.
#[derive(Clone, Debug)]
struct HtmlResource;

/// TODO: Dox
#[derive(Debug, Response)]
#[web(template = "foo")]
struct MyResponse {
    foo: usize,
    bar: &'static str,
}

impl_web! {
    impl HtmlResource {
        // `serde_json::Value` may be used as the response type. In this case,
        // the response will have a content-type of "application/json".
        //
        #[get("/")]
        #[content_type("json")]
        fn hello_world(&self) -> Result<MyResponse, ()> {
            Ok(MyResponse {
                foo: 123,
                bar: "hello world!",
            })
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HtmlResource)
        .run(&addr)
        .unwrap();
}
