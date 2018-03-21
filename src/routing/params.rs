use std::ops;

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
    pub fn get(&self, index: usize) -> &[u8] {
        self.params[index].as_ref()
    }
}

impl ops::Index<usize> for Params {
    type Output = str;

    fn index(&self, index: usize) -> &str {
        &self.params[index]
    }
}
