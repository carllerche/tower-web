mod empty_impl {
    use tower_web::impl_web;

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
    use tower_web::impl_web;

    #[derive(Clone)]
    struct Empty;

    impl_web! {
        impl Empty {
            fn foo(&self) {
            }
        }
    }

    #[test]
    fn use_type() {
        let v = Empty;
        v.foo();
    }
}

mod other_attr {
    use tower_web::impl_web;

    #[derive(Clone)]
    struct Empty;

    impl_web! {
        impl Empty {
            #[inline]
            fn foo(&self) {
            }
        }
    }

    #[test]
    fn use_type() {
        let v = Empty;
        v.foo();
    }
}

mod one_route {
    use tower_web::impl_web;

    #[derive(Clone)]
    struct OneRoute;

    impl_web! {
        impl OneRoute {
            #[get("/")]
            fn foo(&self) -> Result<String, ()> {
                Ok("foo".to_string())
            }
        }
    }

    #[test]
    fn use_type() {
        let v = OneRoute;
        assert_eq!("foo", v.foo().unwrap());
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
