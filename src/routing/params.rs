#[derive(Debug)]
pub(crate) struct Params {
    /// Parameters extracted from the requet
    params: Vec<(usize, usize)>,
}

impl Params {
    pub(crate) fn new(params: Vec<(usize, usize)>) -> Params {
        Params { params }
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.params.len()
    }

    /// Get a parameter value
    pub fn get<'a>(&self, index: usize, src: &'a str) -> &'a str {
        let (pos, len) = self.params[index];
        &src[pos..(pos + len)]
    }
}
