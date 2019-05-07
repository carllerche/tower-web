
macro_rules! expand {
    ($($tt:tt)*) => {{
        let input: ::syn::DeriveInput = ::syn::parse2(quote!($($tt)*)).unwrap();
        crate::derive::expand_derive_response(input)
    }}
}

#[test]
fn invalid_static_status_1() {
    let err = expand! {
        #[web(status)]
        struct Foo { }
    }.unwrap_err();

    assert!(err.contains("invalid struct level `status` annotation"), "actual={}", err)
}

#[test]
fn invalid_static_status_2() {
    let err = expand! {
        #[web(status(code = "201"))]
        struct Foo { }
    }.unwrap_err();

    assert!(err.contains("invalid struct level `status` annotation"), "actual={}", err)
}

#[test]
fn invalid_field_status_1() {
    let err = expand! {
        struct Foo {
            #[web(status = "201")]
            status: u16,
        }
    }.unwrap_err();

    assert!(err.contains("invalid field level `status` annotation"), "actual={}", err)
}

#[test]
fn invalid_field_status_2() {
    let err = expand! {
        #[web(status = "201")]
        struct Foo {
            #[web(status)]
            status: u16,
        }
    }.unwrap_err();

    assert!(err.contains("duplicate `status` annotation"), "actual={}", err)
}

#[test]
fn invalid_field_status_3() {
    let err = expand! {
        struct Foo {
            #[web(status)]
            status_1: u16,

            #[web(status)]
            status_2: u16,
        }
    }.unwrap_err();

    assert!(err.contains("duplicate field level `status` annotation"), "actual={}", err)
}

#[test]
fn invalid_enum_either_multiple_fields() {
    let err = expand! {
        #[web(either)]
        enum Foo {
            Page(String, i32),
            SomethingElse(String),
        }
    }.unwrap_err();

    assert!(err.contains("only single-field variants are supported for `#[web(either)]`"), "actual={}", err)
}

#[test]
fn invalid_enum_either_named_fields() {
    let err = expand! {
        #[web(either)]
        enum Foo {
            Page {
                contents: String
            },
            SomethingElse {
                foo: String
            },
        }
    }.unwrap_err();

    assert!(err.contains("only unnamed fields are supported for `#[web(either)]`"), "actual={}", err)
}

#[test]
#[ignore]
fn invalid_enum_mixed_fields() {
    let err = expand! {
        #[web(either)]
        enum Foo {
            Page {
                contents: String
            },
            SomethingElse(String),
        }
    }.unwrap_err();

    assert!(err.contains("only unnamed fields are supported for `#[web(either)]`"), "actual={}", err)
}

#[test]
fn invalid_struct_either() {
    let err = expand! {
        #[web(either)]
        struct Foo {
            contents: String
        }
    }.unwrap_err();

    assert!(err.contains("only enums are supported for `#[web(either)]`"), "actual={}", err)
}

#[test]
fn invalid_attributes_either() {
    let err = expand! {
        #[web(either)]
        #[web(status = "200")]
        enum Foo {
            String(String)
        }
    }.unwrap_err();

    assert!(err.contains("`#[web(either)]` cannot be used together with other `#[web]` attributes"), "actual={}", err)
}

#[test]
fn template_attr_without_value() {
    let err = expand! {
        #[web(template)]
        struct Foo {}
    }.unwrap_err();

    assert!(err.contains("`#[web(template = \"foo\")]`"), "actual={}", err)
}

#[test]
fn template_attr_on_field() {
    let err = expand! {
        struct Foo {
            #[web(template = "foo")]
            foo: &'static str,
        }
    }.unwrap_err();

    assert!(err.contains("`template` attribute must be at the struct level"), "actual={}", err)
}

/*

   # Invalid

   * serde remote annotation w/ derive response

 */

