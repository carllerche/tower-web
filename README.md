# Tower Web

A web framework for Rust with a focus on removing boilerplate.

[![Build Status](https://travis-ci.org/carllerche/tower-web.svg?branch=master)](https://travis-ci.org/carllerche/tower-web)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/tower-web.svg?maxAge=2592000)](https://crates.io/crates/tower-web)
[![Gitter](https://badges.gitter.im/tower-rs/tower.svg)](https://gitter.im/tower-rs/tower)

[API Documentation][dox]

Tower Web is:

* **Fast**: Fully asynchronous, built on [Tokio] and [Hyper].
* **Ergonomic**: Tower-web decouples HTTP from your application logic, removing
  all boilerplate.
* **Works on Rust stable**: You can use it today.

[Tokio]: https://github.com/tokio-rs/tokio
[Hyper]: http://github.com/hyperium/hyper
[dox]: https://docs.rs/tower-web/0.3.6/tower_web/

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
        #[get("/")]
        #[content_type("json")]
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
