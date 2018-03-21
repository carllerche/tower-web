use bytes::Bytes;

#[derive(Debug)]
pub struct Params {
    /// Parameters extracted from the requet
    ///
    /// TODO: More efficient extraction.
    params: Vec<Bytes>,
}

impl Params {
    /// Get a parameter value
    pub fn get(&self, index: usize) -> &[u8] {
        self.params[index].as_ref()
    }
}
