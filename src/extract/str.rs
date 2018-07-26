use extract::{Extract, Error, Context, Immediate};
use util::BufStream;

impl<B: BufStream> Extract<B> for String {
    type Future = Immediate<String>;

    fn extract(ctx: &Context) -> Self::Future {
        use codegen::Source::*;

        // Get the parameter index from the callsite info
        match ctx.callsite().source() {
            Param(idx) => {
                let path = ctx.request().uri().path();
                let value = ctx.params().get(*idx, path)
                    .to_string();

                Immediate::ok(value)
            }
            Header(header_name) => {
                let value = match ctx.request().headers().get(header_name) {
                    Some(value) => value,
                    None => {
                        return Immediate::err(Error::missing_param());
                    }
                };

                match value.to_str() {
                    Ok(s) => Immediate::ok(s.to_string()),
                    Err(_) => Immediate::err(Error::invalid_param(&"invalid UTF-8 string")),
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
