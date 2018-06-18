#[derive(Debug)]
pub struct Params<'a> {
    /// Parameters extracted from the requet
    params: Vec<&'a str>,
}

impl<'a> Params<'a> {
    pub(crate) fn new(params: Vec<&'a str>) -> Params<'a> {
        Params { params }
    }

    pub fn len(&self) -> usize {
        self.params.len()
    }

    /// Get a parameter value
    pub fn get(&self, index: usize) -> Option<&str> {
        self.params.get(index).map(|s| *s)
    }
}
