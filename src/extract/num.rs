use crate::extract::{Extract, Error, Context, Immediate};
use crate::util::BufStream;

use atoi::atoi;
use checked::Checked;

use std::error::Error as E;
use std::str::FromStr;

macro_rules! num_extract_impls {
    ($($num:ident),+) => {
        $(
            impl<B: BufStream> Extract<B> for $num {
                type Future = Immediate<$num>;

                fn extract(ctx: &Context) -> Self::Future {
                    use crate::codegen::Source::*;

                    match ctx.callsite().source() {
                        Capture(idx) => {
                            let path = ctx.request().uri().path();
                            let capture = ctx.captures().get(*idx, path);

                            $num::from_str(capture).map_err(|err| {
                                Error::invalid_argument(&err.description())
                            }).into()
                        }
                        Header(header_name) => {
                            let value = match ctx.request().headers().get(header_name) {
                                Some(value) => value,
                                None => {
                                    return Immediate::err(Error::missing_argument());
                                }
                            };

                            match atoi(value.as_bytes()) {
                                Some(Checked(Some(s))) => Immediate::ok(s),
                                _ => Immediate::err(Error::invalid_argument(&"invalid integer")),
                            }
                        }
                        QueryString => {
                            unimplemented!();
                        }
                        Body => {
                            unimplemented!();
                        }
                        Unknown => {
                            unimplemented!();
                        }
                    }
                }
            }
        )+
    }
}

// i128,u128 don't work because "no implementation for `checked::Checked<i128> * checked::Checked<i128>"
num_extract_impls!(u8, u16, u32, u64, i8, i16, i32, i64);