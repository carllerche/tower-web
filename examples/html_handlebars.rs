/// Web service that receives and responds with HTML.
///
/// ## Overview
///
/// Tower web supports templates and responding with HTML using the handlebars
/// templating engine. Plain old Rust structs are used to represent data and
/// are used as the handler return value. Tower Web passes the response structs
/// to the handlebars serializer. HTML is rendered using a handlebars template
/// and populated using the data in the response struct.
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

/// The type is annotated with `#[derive(Response)]`, this allows `MyResponse`
/// to be used as a response to resource methods.
///
/// We are using the handlebars serializer to render the HTML response. It
/// requires that a template to render is specified. This is done with the
/// `#[web(template = "<template name>")]` attribute.
///
/// The default location to look for templates is in the `templates` directory
/// at the crate root. To make the example work, this crate has a handlebars
/// template at "foo.hbs" in the templates directory at the crate root.
#[derive(Debug, Response)]
struct MyResponse {
    title: &'static str,
}

impl_web! {
    impl HtmlResource {
        // Respond as HTML. For this to work, a serializer supporting HTML must
        // be added to the service.
        //
        // If no serializer is specified, a 500 response will be returned.
        //
        #[get("/")]
        #[content_type("html")]
        #[web(template = "examples/hello_world")]
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
        // Add the handlebars serializer to the application. This uses the
        // template rendering default settings. Templates are located at
        // the crate root in the `templates` directory. Template files
        // use the `.hbs` extension.
        //
        // The handlebars serializer is configured by calling
        // `Handlebars::new_with_registry` and passing in a configured
        // registry. This allows changing the template directory as well
        // as defining helpers and other configuration options.
        //
        // See the `handlebars` crate for more documentation on configuration
        // options.
        .serializer(Handlebars::new())
        .run(&addr)
        .unwrap();
}
