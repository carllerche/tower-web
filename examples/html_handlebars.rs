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

extern crate env_logger;
#[macro_use]
extern crate tower_web;
extern crate tokio;

use tower_web::ServiceBuilder;
use tower_web::view::Handlebars;

/// This type will be the web service implementation.
#[derive(Clone, Debug)]
struct HtmlResource;

/// TODO: Dox
#[derive(Debug, Response)]
#[web(template = "foo")]
struct MyResponse {
    title: &'static str,
}

impl_web! {
    impl HtmlResource {
        // `serde_json::Value` may be used as the response type. In this case,
        // the response will have a content-type of "application/json".
        //
        #[get("/")]
        #[content_type("html")]
        fn hello_world(&self) -> Result<MyResponse, ()> {
            Ok(MyResponse {
                title: "Handler variable",
            })
        }
    }
}

pub fn main() {
    let _ = env_logger::try_init();

    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HtmlResource)
        .serializer(Handlebars::new())
        .run(&addr)
        .unwrap();
}
