# Tower Web

A web framework for Rust with a focus on removing boilerplate.

[![Join the chat at https://gitter.im/tower-rs/tower](https://badges.gitter.im/tower-rs/tower.svg)](https://gitter.im/tower-rs/tower?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

Tower Web is:

* **Fast**: Fully asynchronous, built on [Tokio] and [Hyper].
* **Ergonomic**: Tower-web decouples HTTP from your application logic, removing
  all boilerplate.
* **Works on Rust stable**: You can use it today.

[Tokio]: https://github.com/tokio-rs/tokio
[Hyper]: http://github.com/hyperium/hyper

## Hello World

```rust
#[macro_use]
extern crate tower_web;
extern crate tokio;

use tower_web::ServiceBuilder;
use tokio::prelude::*;

/// This type will be part of the web service as a resource.
#[derive(Clone, Debug)]
struct HelloWorld;

/// This will be the JSON response
#[derive(Response)]
struct HelloResponse {
    message: &'static str,
}

impl_web! {
    impl HelloWorld {
        /// @get("/")
        /// @content_type("json")
        fn hello_world(&self) -> Result<HelloResponse, ()> {
            Ok(HelloResponse {
                message: "hello world",
            })
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HelloWorld)
        .run(&addr)
        .unwrap();
}
```

## Overview

Tower Web aims to decouple all HTTP concepts from the application logic. You
define a "plain old Rust method" (PORM?). This method takes only the data it
needs to complete and returns a struct representing the response. Tower Web does
the rest.

The `impl_web` macro looks at the definition and generates the glue code,
allowing the method to respond to HTTP requests.

In order to work on Rust stable **today**, Tower Web uses doc comments as
attributes. Once Macro 1.2 lands, this will be switched to real attributes.

## Getting Started

The best way to get started is to read the [examples] and [API docs][dox].

[dox]: #
[examples]: examples/

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `tower-web` by you, shall be licensed as MIT, without any
additional terms or conditions.
