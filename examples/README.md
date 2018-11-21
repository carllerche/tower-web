# `tower-web` usage examples.

This directory contains a number of examples showcasing various capabilities of
`tower-web`.

All examples can be executed with:

```
cargo run --example $name
```

It is recommended to explore the examples in (approximately) the following
order:

* [`hello_world`](hello_world.rs) - getting started with `tower_web`. This
  demonstrates how to get a basic web service running.

* [`args`](args.rs) - Handler arguments are populated using the HTTP request.

* [`derive_extract`](derive_extract.rs) - Custom type handler arguments are
  populated using the HTTP request.

* [`json`](json.rs) - Receiving and responding with JSON. This example also
  shows how to customize the HTTP response status and headers.

* [`static_file`](static_file.rs) - Respond with static files from disk. This
  examplee also shows glob path parameteres.

* [`middleware`](middleware.rs) - Decorate the application with middleware.
  Doing so adds additional functionality. This example adds request logging.

* [`html_handlebars`](html_handlebars.rs) - Respond with HTML by rendering
  handlebars templates.

Tower Web provides experimental support for Rust's `async` / `await`
syntax. To use this syntax, the Rust nightly release channel is required
and the crate must be set to the 2018 edition.

The example in the [`async-await`] directory contains a [`Cargo.toml`]
as well as code.

1) Add [`edition = 2018`][2018] to your `Cargo.toml`.
2) Add [`features = ["async-await-preview"]`][feature] to the
`tower-web` dependency.
3) Use the necessary [nightly features] in the application.
4) Import Tokio's [`await!` macro][await].
5) Define [`async`][async-handler] handlers.

Support for serving data over TLS is provided with the rustls feature.
The [`rustls`](rustls) directory contains an example along with a
[Cargo.toml](ruslts/Cargo.toml) file.

1) Add [`features = ["rustls"]`](rustls/Cargo.toml) to the `tower-web` dependency.
2) Import [tokio-rustls](https://crates.io/crates/tokio-rustls).
3) Configure [TLSAcceptor](rustls/src/main.rs#L47).
4) Wrap [incoming TcpStream](ruslts/src/main.rs#66) and handle errors

[`async-await`]: async-await
[`Cargo.toml`]: async-await/Cargo.toml
[2018]: async-await/Cargo.toml
[feature]: async-await/Cargo.toml
[nightly features]: async-await/src/hyper.rs#L22
[await]: async-await/src/hyper.rs#L30
[async-handler]: async-await/src/hyper.rs#L54
