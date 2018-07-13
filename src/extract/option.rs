use super::{CallSiteExtract, Error};

use codegen::CallSite;
use routing::RouteMatch;

use http::Request;

impl<'a, T> CallSiteExtract<'a> for Option<T>
where T: CallSiteExtract<'a>
{
    /// TODO: Dox
    fn callsite_extract(
        callsite: &CallSite,
        route: &'a RouteMatch,
        request: &'a Request<()>,
    ) -> Result<Self, Error> {
        match T::callsite_extract(callsite, route, request) {
            Ok(v) => Ok(Some(v)),
            Err(ref e) if e.is_missing_param() => Ok(None),
            Err(e) => Err(e),
        }
    }
}
