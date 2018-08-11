use chrono::{DateTime, Timelike, Utc};
use extract::{Context, Error, Extract, Immediate};
use http::{self, header};
use std::time::SystemTime;
use util::buf_stream::BufStream;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct HttpDateTime(DateTime<Utc>);

impl HttpDateTime {
    fn normalize(dt: DateTime<Utc>) -> Self {
        // We don't care about anything smaller than a second
        let dt = dt
            .with_nanosecond(0)
            .expect("Unable to normalize HttpDateTime");
        HttpDateTime(dt)
    }
}

impl From<SystemTime> for HttpDateTime {
    fn from(t: SystemTime) -> Self {
        HttpDateTime::normalize(t.into())
    }
}

impl http::HttpTryFrom<HttpDateTime> for header::HeaderValue {
    type Error = header::InvalidHeaderValue;

    fn try_from(t: HttpDateTime) -> Result<header::HeaderValue, Self::Error> {
        let s = t.0.to_rfc2822();
        http::HttpTryFrom::try_from(s.as_str())
    }
}

impl<B: BufStream> Extract<B> for HttpDateTime {
    type Future = Immediate<HttpDateTime>;

    fn extract(ctx: &Context) -> Self::Future {
        use codegen::Source::*;

        match ctx.callsite().source() {
            Header(header_name) => {
                let value = match ctx.request().headers().get(header_name) {
                    Some(value) => value,
                    None => return Immediate::err(Error::missing_param()),
                };

                let value = match value.to_str() {
                    Ok(s) => s,
                    Err(_) => return Immediate::err(Error::invalid_param(&"invalid UTF-8 string")),
                };

                match DateTime::parse_from_rfc2822(&value) {
                    Ok(dt) => Immediate::ok(HttpDateTime::normalize(dt.with_timezone(&Utc))),
                    Err(e) => Immediate::err(Error::invalid_param(&e)),
                }
            }
            _ => unimplemented!("A HttpDateTime can only be extracted from the headers for now"),
        }
    }
}
