use http::header::{self, HeaderName};

#[derive(Debug)]
pub struct CallSite {
    /// The argument name
    arg: &'static str,

    /// If the arg name matches a header name, this field is populated with the
    /// header.
    header_name: Option<HeaderName>,

    /// Param index
    param: Option<usize>,
}

impl CallSite {
    // TODO: This should probably be a builder
    pub fn new(arg: &'static str, param: Option<usize>) -> CallSite {
        let header_name = match arg {
            "content_type" => Some(header::CONTENT_TYPE),
            "user_agent" => Some(header::USER_AGENT),
            _ => None,
        };

        CallSite {
            arg,
            header_name,
            param
        }
    }

    /*
    /// TODO: Dox
    pub(crate) fn arg(&self) -> &'static str {
        self.arg
    }
    */

    pub(crate) fn header_name(&self) -> Option<&HeaderName> {
        self.header_name.as_ref()
    }

    pub(crate) fn param(&self) -> Option<usize> {
        self.param
    }
}
