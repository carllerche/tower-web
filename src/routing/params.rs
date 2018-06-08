#[derive(Debug)]
pub struct Params {
    /// Parameters extracted from the requet
    ///
    /// TODO: More efficient extraction.
    params: Vec<String>,
}

impl Params {
    pub(crate) fn new(params: Vec<String>) -> Params {
        Params { params }
    }

    pub fn len(&self) -> usize {
        self.params.len()
    }

    /// Get a parameter value
    pub fn get(&self, index: usize) -> Option<&[u8]> {
        self.params.get(index).map(|s| s.as_ref())
    }
}
