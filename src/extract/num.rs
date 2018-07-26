use extract::{Extract, Error, Context, Immediate};
use util::BufStream;

use atoi::atoi;

use std::error::Error as E;
use std::str::FromStr;

impl<B: BufStream> Extract<B> for u32 {
    type Future = Immediate<u32>;

    fn extract(ctx: &Context) -> Self::Future {
        use codegen::Source::*;

        // Get the parameter index from the callsite info
        match ctx.callsite().source() {
            Param(idx) => {
                let path = ctx.request().uri().path();
                let param = ctx.params().get(*idx, path);

                u32::from_str(param).map_err(|err| {
                    Error::invalid_param(&err.description())
                }).into()
            }
            Header(header_name) => {
                let value = match ctx.request().headers().get(header_name) {
                    Some(value) => value,
                    None => {
                        return Immediate::err(Error::missing_param());
                    }
                };

                match atoi(value.as_bytes()) {
                    Some(s) => Immediate::ok(s),
                    None => Immediate::err(Error::invalid_param(&"invalid integer")),
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
