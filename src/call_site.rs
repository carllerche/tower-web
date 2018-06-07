#[derive(Debug)]
pub struct CallSite {
    /// The argument name
    arg: &'static str,

    /// Param index
    param: Option<usize>,
}

impl CallSite {
    // TODO: This should probably be a builder
    pub fn new(arg: &'static str, param: Option<usize>) -> CallSite {
        CallSite { arg, param }
    }

    pub(crate) fn arg(&self) -> &'static str {
        self.arg
    }

    pub(crate) fn param(&self) -> Option<usize> {
        self.param
    }
}
