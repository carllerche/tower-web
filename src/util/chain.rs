/// Combine two values
///
/// This trait is used to represent types that can be chained, including
/// middleware and resource values.
pub trait Chain<U> {
    /// The combined type
    type Output;

    /// Combine `self` with `other`.
    fn chain(self, other: U) -> Self::Output;
}
