use super::{CallSiteExtract, Error};

use codegen::CallSite;
use routing::RouteMatch;

use http::Request;

impl<'a> CallSiteExtract<'a> for String {
    fn callsite_extract(
        callsite: &CallSite,
        route_match: &'a RouteMatch,
        request: &'a Request<()>,
    ) -> Result<Self, Error> {
        <&str as CallSiteExtract>::callsite_extract(callsite, route_match, request)
            .map(|s| s.to_string())
    }
}

impl<'a> CallSiteExtract<'a> for &'a str {
    fn callsite_extract(
        callsite: &CallSite,
        route_match: &'a RouteMatch,
        request: &'a Request<()>,
    ) -> Result<Self, Error> {
        unimplemented!();
        /*
        use std::error::Error as E;

        // Get the parameter index from the callsite info
        match callsite.param() {
            Some(idx) => {
                let param = route_match.params().get(idx)
                    .expect("parameter missing");

                Ok(param)
            }
            None => {
                let val = match request.headers().get(callsite.header_name()) {
                    Some(val) => val,
                    None => return Err(Error::missing_param()),
                };

                match val.to_str() {
                    Ok(s) => Ok(s),
                    Err(e) => Err(Error::invalid_param(&e.description())),
                }
            }
        }
        */
    }
}
