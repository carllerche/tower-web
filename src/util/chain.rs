/// Combine two values
pub trait Chain<U> {
    type Output;

    fn chain(self, other: U) -> Self::Output;
}
