
pub struct Chain<T, U> {
    left: T,
    right: U,
}

impl<T, U> Chain<T, U> {
    pub(crate) fn new(left: T, right: U) -> Chain<T, U> {
        Chain {
            left,
            right,
        }
    }
}
