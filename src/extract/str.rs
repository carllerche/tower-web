use extract::{Extract, Error, Context, Immediate};
use util::BufStream;

impl<B: BufStream> Extract<B> for String {
    type Future = Immediate<String>;

    fn extract(ctx: &Context) -> Self::Future {
        use codegen::Source::*;

        match ctx.callsite().source() {
            Capture(idx) => {
                let path = ctx.request().uri().path();
                let value = ctx.captures().get(*idx, path)
                    .to_string();

                Immediate::ok(value)
            }
            Header(header_name) => {
                let value = match ctx.request().headers().get(header_name) {
                    Some(value) => value,
                    None => {
                        return Immediate::err(Error::missing_argument());
                    }
                };

                match value.to_str() {
                    Ok(s) => Immediate::ok(s.to_string()),
                    Err(_) => Immediate::err(Error::invalid_argument(&"invalid UTF-8 string")),
                }
            }
            QueryString => {
                let query = ctx.request().uri()
                    .path_and_query()
                    .and_then(|path_and_query| path_and_query.query())
                    .unwrap_or("");

                Immediate::ok(query.to_string())
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
