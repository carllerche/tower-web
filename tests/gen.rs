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

/*
#[derive(Clone)]
struct Empty;

impl_web! {
    impl Empty {

        // These should not parse due to an invalid fn sig

        /// @GET("/")
        fn foo() {
        }

        /// @GET("/:id")
        fn foo(id: u32) {
        }

        /// @GET("/")
        fn foo(self) {
        }

        /// @GET("/:id")
        fn foo(self, id: u32) {
        }

        /// @GET("/")
        fn foo(self: Box<Self>) {
        }

        /// @GET("/:id")
        fn foo(self: Box<Self>, id: u32) {
        }
    }
}
 */

// Additional tests:
// * Passing arg that does not impl `Extract`
// * No function generics
