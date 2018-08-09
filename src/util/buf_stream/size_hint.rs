//! `SizeHint` type and builder

/// A `BufStream` size hint
///
/// The default implementation returns:
///
/// * 0 for `available`
/// * 0 for `lower`
/// * `None` for `upper`.
#[derive(Debug, Default, Clone)]
pub struct SizeHint {
    available: usize,
    lower: usize,
    upper: Option<usize>,
}

/// Build a `SizeHint`
#[derive(Debug)]
pub struct Builder {
    hint: SizeHint,
}

impl Builder {
    /// Create a new `Builder` configured with default values.
    pub fn new() -> Builder {
        Builder {
            hint: SizeHint {
                available: 0,
                lower: 0,
                upper: None,
            },
        }
    }

    /// Sets the `available` hint value.
    pub fn available(&mut self, val: usize) -> &mut Self {
        self.hint.available = val;

        if self.hint.lower < val {
            self.hint.lower = val;

            match self.hint.upper {
                Some(ref mut upper) if *upper < val => {
                    *upper = val;
                }
                _ => {}
            }
        }

        self
    }

    /// Sets the `lower` hint value.
    ///
    /// # Panics
    ///
    /// This function panics if `val` is smaller than `available`.
    pub fn lower(&mut self, val: usize) -> &mut Self {
        assert!(val >= self.hint.available);

        self.hint.lower = val;
        self
    }

    /// Set the `upper` hint value.
    ///
    /// # Panics
    ///
    /// This function panics if `val` is smaller than `lower`.
    pub fn upper(&mut self, val: usize) -> &mut Self {
        // There is no need to check `available` as that is guaranteed to be
        // less than or equal to `lower`.
        assert!(val >= self.hint.lower, "`val` is smaller than `lower`");

        self.hint.upper = Some(val);
        self
    }

    /// Build the `SizeHint`
    pub fn build(&self) -> SizeHint {
        self.hint.clone()
    }
}

impl SizeHint {
    /// Returns the **lower bound** of the amount of data that can be read from
    /// the `BufStream` without `NotReady` being returned.
    ///
    /// It is possible that more data is currently available.
    pub fn available(&self) -> usize {
        self.available
    }

    /// Returns the lower bound of data that the `BufStream` will yield before
    /// completing.
    pub fn lower(&self) -> usize {
        self.lower
    }

    /// Returns the upper bound of data the `BufStream` will yield before
    /// completing, or `None` if the value is unknown.
    pub fn upper(&self) -> Option<usize> {
        self.upper
    }
}
