use extract::{Extract, Error, Context, Immediate};
use util::BufStream;

use atoi::atoi;
use checked::Checked;

use std::error::Error as E;
use std::str::FromStr;

impl<B: BufStream> Extract<B> for u32 {
    type Future = Immediate<u32>;

    fn extract(ctx: &Context) -> Self::Future {
        use codegen::Source::*;

        match ctx.callsite().source() {
            Capture(idx) => {
                let path = ctx.request().uri().path();
                let capture = ctx.captures().get(*idx, path);

                u32::from_str(capture).map_err(|err| {
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
