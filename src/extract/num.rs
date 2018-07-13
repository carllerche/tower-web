use extract::{Extract, Error, Context, Immediate};

use atoi::atoi;

use std::error::Error as E;
use std::str::FromStr;

impl Extract for u32 {
    type Future = Immediate<u32>;

    fn into_future(ctx: &Context) -> Self::Future {
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
            Header(idx) => {
                unimplemented!();
                /*
                // Get the header name for the argument.
                let header_name = &route_match.header_names()[idx];

                let val = match request.headers().get(header_name) {
                    Some(val) => val,
                    None => return Err(Error::missing_param()),
                };

                match atoi(val.as_bytes()) {
                    Some(s) => Ok(s),
                    None => Err(Error::invalid_param(&"not valid integer")),
                }
                */
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
