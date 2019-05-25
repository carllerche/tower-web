use quote::quote;

macro_rules! expand {
    ($($tt:tt)*) => {{
        crate::resource::expand_derive_resource(quote!($($tt)*))
    }}
}

#[test]
#[should_panic(expected = "duplicate routes with method")]
fn duplicate_routes(){
    expand! {
        impl Test{
            #[get("/foo")]
            #[content_type("plain")]
            fn foo(&self){}

            #[get("/foo")]
            #[content_type("plain")]
            fn bar(&self){}
        }
    };
}
