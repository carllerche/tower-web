use self::Source::*;

use http::header::HeaderName;

#[derive(Debug)]
pub struct CallSite {
    /// Where to extract the parameter from when the argument type does not
    /// provide the information.
    source: Source,
}

#[derive(Debug, Clone)]
pub(crate) enum Source {
    Param(usize),
    Header(HeaderName),
    QueryString,
    Body,
    Unknown,
}

impl CallSite {
    pub fn new_param(index: usize) -> CallSite {
        CallSite { source: Param(index) }
    }

    pub fn new_header(name: &'static str) -> CallSite {
        CallSite { source: Header(HeaderName::from_static(name)) }
    }

    pub fn new_query_string() -> CallSite {
        CallSite { source: QueryString }
    }

    pub fn new_body() -> CallSite {
        CallSite { source: Body }
    }

    /// Cannot infer where to extract the parameter based on the call site.
    pub fn new_unknown() -> CallSite {
        CallSite { source: Unknown }
    }

    pub(crate) fn source(&self) -> &Source {
        &self.source
    }
}
