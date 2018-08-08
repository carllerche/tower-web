#[derive(Debug)]
pub(crate) struct Captures {
    /// Captures extracted from the request
    captures: Vec<(usize, usize)>,
}

impl Captures {
    pub(crate) fn new(captures: Vec<(usize, usize)>) -> Captures {
        Captures { captures }
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.captures.len()
    }

    /// Get a capture
    pub fn get<'a>(&self, index: usize, src: &'a str) -> &'a str {
        let (pos, len) = self.captures[index];
        &src[pos..(pos + len)]
    }
}
