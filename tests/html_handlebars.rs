#![cfg(feature = "handlebars")]

extern crate env_logger;
extern crate futures;
extern crate handlebars;
extern crate http;
#[macro_use]
extern crate tower_web;

#[macro_use]
mod support;
use support::*;

use handlebars::Handlebars as Registry;
use tower_web::view::Handlebars;

#[derive(Debug)]
struct TestHandlebars;

#[derive(Response, Debug)]
#[web(template = "foo")]
struct Foo {
    title: &'static str,
}

#[derive(Response, Debug)]
struct Bar {
    title: &'static str,
}

#[derive(Response, Debug)]
#[web(template = "missing")]
struct BadTemplate {
    title: &'static str,
}

#[derive(Response, Debug)]
struct NoTemplate {
    title: &'static str,
}

impl_web! {
    impl TestHandlebars {
        #[get("/foo")]
        #[content_type("html")]
        fn foo(&self) -> Result<Foo, ()> {
            Ok(Foo {
                title: "hello",
            })
        }

        #[get("/bar")]
        #[content_type("html")]
        #[web(template = "bar")]
        fn bar(&self) -> Result<Bar, ()> {
            Ok(Bar {
                title: "world",
            })
        }

        #[get("/foo2")]
        #[content_type("html")]
        #[web(template = "bar")]
        fn foo2(&self) -> Result<Foo, ()> {
            // If both the handler and the response have specified a template, the response value
            // takes precedence.
            Ok(Foo {
                title: "2",
            })
        }

        #[get("/bad_template")]
        #[content_type("html")]
        fn bad_template(&self) -> Result<BadTemplate, ()> {
            Ok(BadTemplate {
                title: "This is foo",
            })
        }

        #[get("/no_template")]
        #[content_type("html")]
        fn no_template(&self) -> Result<NoTemplate, ()> {
            Ok(NoTemplate {
                title: "This is foo",
            })
        }
    }
}

#[test]
fn render_template_response_attr() {
    let mut web = service_with_serializer(TestHandlebars, hb());

    let response = web.call_unwrap(get!("/foo"));
    assert_ok!(response);
    assert_header!(response, "content-type", "text/html");
    assert_body!(response, "<html><title>Foo - hello</title></html>\n");
}

#[test]
fn render_template_handler_attr() {
    let mut web = service_with_serializer(TestHandlebars, hb());

    let response = web.call_unwrap(get!("/bar"));
    assert_ok!(response);
    assert_header!(response, "content-type", "text/html");
    assert_body!(response, "<html><title>Bar - world</title></html>\n");
}

#[test]
fn render_template_both_handler_and_response_attr() {
    let mut web = service_with_serializer(TestHandlebars, hb());

    let response = web.call_unwrap(get!("/foo2"));
    assert_ok!(response);
    assert_header!(response, "content-type", "text/html");
    assert_body!(response, "<html><title>Foo - 2</title></html>\n");
}

#[test]
fn invalid_template() {
    let mut web = service_with_serializer(TestHandlebars, hb());

    let response = web.call_unwrap(get!("/bad_template"));
    assert_internal_error!(response);
}

#[test]
fn no_template() {
    let mut web = service_with_serializer(TestHandlebars, hb());

    let response = web.call_unwrap(get!("/no_template"));
    assert_internal_error!(response);
}

fn hb() -> Handlebars {
    use std::env;
    use std::path::Path;

    let mut registry = Registry::new();

    let dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("tests/templates/hbs");

    registry.register_templates_directory(".hbs", dir).unwrap();

    Handlebars::new_with_registry(registry)
}
