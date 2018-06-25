
/// A `BufStream` size hint
#[derive(Debug)]
pub struct SizeHint {
    available: usize,
    lower: usize,
    upper: Option<usize>,
}

impl SizeHint {
    pub fn available(&self) -> usize {
        self.available
    }

    pub fn lower(&self) -> usize {
        self.lower
    }

    pub fn upper(&self) -> Option<usize> {
        self.upper
    }
}
