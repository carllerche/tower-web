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
