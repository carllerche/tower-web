/// Web service that receives and responds with JSON.
///
/// ## Overview
///
/// Tower Web uses serde under the hood to provide serialization and
/// deserialization. Plain old Rust structs are used to represent data and Tower
/// Web ensures that they are deserialized from the request and serialized when
/// responding.
///
/// This example also shows how to customize the HTTP response status and
/// headers.
///
/// ## Usage
///
/// Run the example:
///
///     cargo run --example json
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
struct JsonResource;

/// This type is annotated with `#[derive(Response)]`, this allows `MyResponse`
/// to be used as a response to a resource method.
///
/// `#[derive(Response)]` delegates to serde's `#[derive(Serialize)]` under the
/// hood. As such, all of serde's annotations can be used here as well. However,
/// as we will see shortly, Tower Web provides some additional annotations to
/// use as well.
#[derive(Debug, Response)]
struct MyResponse {
    foo: usize,
    bar: &'static str,
}

/// This is another response type that will be serialized to JSON. However, here
/// we customize the HTTP status code. When `CreatedResponse` is returned, the
/// HTTP status code will be set to 201.
#[derive(Debug, Response)]
#[web(status = "201")]
struct CreatedResponse {
    message: &'static str,

    /// This specifies that the value of this field should be set as a HTTP
    /// header of the same name (x-my-header).
    #[web(header)]
    x_my_header: &'static str,
}

impl_web! {
    impl JsonResource {
        // `serde_json::Value` may be used as the response type. In this case,
        // the response will have a content-type of "application/json".
        //
        #[get("/")]
        fn hello_world(&self) -> Result<serde_json::Value, ()> {
            Ok(json!({
                "message": "hello world",
            }))
        }

        // Here, a custom type is used as the response type. The struct itself
        // does not imply any sort of content type, so we annotate the action
        // here, indicating that the response should be serialized as JSON.
        //
        #[get("/custom-type")]
        #[content_type("application/json")]
        fn custom_type(&self) -> Result<MyResponse, ()> {
            Ok(MyResponse {
                foo: 123,
                bar: "hello world",
            })
        }

        // Same as above, but this time we also customize the HTTP status code
        // and include a custom header in the response.
        //
        #[post("/create")]
        #[content_type("application/json")]
        fn create(&self) -> Result<CreatedResponse, ()> {
            Ok(CreatedResponse {
                message: "created",
                x_my_header: "awesome",
            })
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(JsonResource)
        .run(&addr)
        .unwrap();
}

