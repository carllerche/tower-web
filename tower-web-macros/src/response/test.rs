
macro_rules! expand {
    ($($tt:tt)*) => {{
        let input: ::syn::DeriveInput = ::syn::parse2(quote!($($tt)*)).unwrap();
        ::response::expand_derive_response(input)
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

/*

   # Invalid

   * serde remote annotation w/ derive response

 */
