#[macro_use]
extern crate tower_web;

mod empty_impl {
    use tower_web::*;

    #[derive(Clone, Debug)]
    struct Empty;

    impl_web! {
        impl Empty {
        }
    }

    #[test]
    fn use_type() {
        let _v = Empty;
    }
}

mod no_routes {
    use tower_web::*;

    #[derive(Clone)]
    struct Empty;

    impl_web! {
        impl Empty {
            fn foo(&mut self) {
            }
        }
    }

    #[test]
    fn use_type() {
        let mut v = Empty;
        v.foo();
    }
}

mod other_attr {
    use tower_web::*;

    #[derive(Clone)]
    struct Empty;

    impl_web! {
        impl Empty {
            #[inline]
            fn foo(&mut self) {
            }
        }
    }

    #[test]
    fn use_type() {
        let mut v = Empty;
        v.foo();
    }
}
