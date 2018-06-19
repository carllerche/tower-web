pub mod tuple;

/// Combine two values
pub trait Chain<U> {
    type Output;

    fn chain(self, other: U) -> Self::Output;
}

pub(crate) mod sealed {
    /// Private trait to this crate to prevent traits from being implemented in
    /// downstream crates.
    pub trait Sealed {}
}
